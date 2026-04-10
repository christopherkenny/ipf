use extendr_api::prelude::*;

/// Kahan summation for better numeric stability.
#[inline]
fn kahan_sum(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    for &x in values {
        let y = x - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

/// One margin encoded with integer levels (0 = ignore) and target totals.
#[derive(Clone)]
struct Margin {
    levels: Vec<usize>, // 0..=L, 0 means "ignore"
    targets: Vec<f64>,  // length L
}

/// Helper: fetch a named element from an R List.
fn list_get(list: &List, name: &str) -> Option<Robj> {
    for (k, v) in list.iter() {
        if k == name {
            return Some(v);
        }
    }
    None
}

/// Parse margins from an R list. Each element must be a list with $levels and $targets.
fn parse_margins(margins: List) -> Result<Vec<Margin>> {
    let mut out = Vec::with_capacity(margins.len());
    for (_name, robj) in margins.iter() {
        let mlist: List = robj
            .try_into()
            .map_err(|_| Error::Other("each margin must be a list".into()))?;

        let levels_robj = list_get(&mlist, "levels")
            .ok_or_else(|| Error::Other("missing $levels".into()))?;
        let targets_robj = list_get(&mlist, "targets")
            .ok_or_else(|| Error::Other("missing $targets".into()))?;

        let li: Integers = levels_robj
            .try_into()
            .map_err(|_| Error::Other("$levels must be integer".into()))?;
        let td: Doubles = targets_robj
            .try_into()
            .map_err(|_| Error::Other("$targets must be numeric".into()))?;

        let levels: Vec<usize> = li
            .iter()
            .map(|x| {
                // NA -> 0, clamp at 0
                let v = if x.is_na() { 0 } else { x.inner() };
                v.max(0) as usize
            })
            .collect();

        let targets: Vec<f64> = td
            .iter()
            .map(|x| if x.is_na() { 0.0 } else { x.inner() })
            .collect();

        out.push(Margin { levels, targets });
    }
    Ok(out)
}

/// Record a diagnostics snapshot without capturing outer borrows.
fn record_diag_snapshot(
    ms: &Vec<Margin>,
    sums: &mut [f64],
    w: &[f64],
    n: usize,
    iter_idx: i32,
    diag_iter: &mut Vec<f64>,
    diag_margin: &mut Vec<f64>,
    diag_level: &mut Vec<f64>,
    diag_target: &mut Vec<f64>,
    diag_current: &mut Vec<f64>,
    diag_err: &mut Vec<f64>,
) {
    for (mi, m) in ms.iter().enumerate() {
        let lcount = m.targets.len();
        if lcount == 0 {
            continue;
        }
        // zero sums
        for s in &mut sums[..lcount] {
            *s = 0.0;
        }
        // group sums
        for i in 0..n {
            let code = m.levels[i];
            if code > 0 {
                sums[code - 1] += w[i];
            }
        }
        // push rows
        for li in 0..lcount {
            let tgt = m.targets[li];
            let cur = sums[li];
            let err = if tgt > 0.0 { (cur - tgt).abs() / tgt } else { cur.abs() };
            diag_iter.push(iter_idx as f64);
            diag_margin.push((mi + 1) as f64);
            diag_level.push((li + 1) as f64);
            diag_target.push(tgt);
            diag_current.push(cur);
            diag_err.push(err);
        }
    }
}

/// Raking / IPF core
#[extendr]
fn rake_ipf_rust(
    weights: Doubles,       // numeric vector length n
    margins: List,          // list of margins (see parse_margins)
    max_iter: i32,          // maximum iterations
    tol: f64,               // convergence tolerance (max proportional error)
    bounds: Robj,           // NULL or numeric length-2 c(lo, hi)
    grand_total: f64,       // total weight to preserve after trimming
    diagnostics_every: i32, // 0 = only baseline; >0 record every k iterations
    verbose: bool,
) -> List {
    // Convert weights (NA -> 0.0). Your R wrapper should already validate.
    let mut w: Vec<f64> = weights.iter().map(|x| if x.is_na() { 0.0 } else { x.inner() }).collect();
    let n = w.len();

    // Parse bounds if present.
    let bounds_opt: Option<(f64, f64)> = if bounds.is_null() {
        None
    } else {
        match bounds.try_into() as std::result::Result<Doubles, _> {
            Ok(v) if v.len() == 2 => {
                let lo = if v[0].is_na() { f64::NAN } else { v[0].inner() };
                let hi = if v[1].is_na() { f64::NAN } else { v[1].inner() };
                if lo.is_finite() && hi.is_finite() && lo <= hi {
                    Some((lo, hi))
                } else {
                    rprintln!("Warning: `bounds` must be finite and ordered (lo <= hi); ignoring.");
                    None
                }
            }
            Ok(v) => {
                if !v.is_empty() {
                    rprintln!("Warning: `bounds` must be numeric length-2; ignoring.");
                }
                None
            }
            Err(_) => {
                rprintln!("Warning: `bounds` must be numeric length-2; ignoring.");
                None
            }
        }
    };

    // Parse margins
    let ms = parse_margins(margins).unwrap();

    // Scratch buffers to largest L
    let max_l = ms.iter().map(|m| m.targets.len()).max().unwrap_or(0);
    let mut sums = vec![0.0f64; max_l];
    let mut factors = vec![1.0f64; max_l];

    // Diagnostics
    let mut diag_iter = Vec::<f64>::new();
    let mut diag_margin = Vec::<f64>::new(); // 1-based
    let mut diag_level = Vec::<f64>::new();  // 1-based
    let mut diag_target = Vec::<f64>::new();
    let mut diag_current = Vec::<f64>::new();
    let mut diag_err = Vec::<f64>::new();

    // Baseline snapshot
    record_diag_snapshot(
        &ms,
        &mut sums,
        &w,
        n,
        0,
        &mut diag_iter,
        &mut diag_margin,
        &mut diag_level,
        &mut diag_target,
        &mut diag_current,
        &mut diag_err,
    );

    // Main loop
    let mut converged = false;
    let mut iterations = 0i32;
    let mut max_prop_err = f64::INFINITY;

    'outer: for it in 1..=max_iter {
        iterations = it;

        for m in &ms {
            let lcount = m.targets.len();
            if lcount == 0 {
                continue;
            }

            // group sums
            for s in &mut sums[..lcount] {
                *s = 0.0;
            }
            for i in 0..n {
                let code = m.levels[i];
                if code > 0 {
                    sums[code - 1] += w[i];
                }
            }

            // factors
            for li in 0..lcount {
                let cur = sums[li];
                factors[li] = if cur > 0.0 { m.targets[li] / cur } else { 1.0 };
            }

            // apply
            for i in 0..n {
                let code = m.levels[i];
                if code > 0 {
                    w[i] *= factors[code - 1];
                }
            }

            // optional bounds + renormalize
            if let Some((lo, hi)) = bounds_opt {
                for wi in &mut w {
                    if *wi < lo {
                        *wi = lo;
                    }
                    if *wi > hi {
                        *wi = hi;
                    }
                }
                let s = kahan_sum(&w);
                if s > 0.0 {
                    let c = grand_total / s;
                    for wi in &mut w {
                        *wi *= c;
                    }
                }
            }
        }

        if diagnostics_every > 0 && (it % diagnostics_every == 0) {
            record_diag_snapshot(
                &ms,
                &mut sums,
                &w,
                n,
                it,
                &mut diag_iter,
                &mut diag_margin,
                &mut diag_level,
                &mut diag_target,
                &mut diag_current,
                &mut diag_err,
            );
        }

        // convergence: max proportional error across all margins
        let mut mpe = 0.0f64;
        for m in &ms {
            let lcount = m.targets.len();
            if lcount == 0 {
                continue;
            }
            for s in &mut sums[..lcount] {
                *s = 0.0;
            }
            for i in 0..n {
                let code = m.levels[i];
                if code > 0 {
                    sums[code - 1] += w[i];
                }
            }
            for li in 0..lcount {
                let tgt = m.targets[li];
                let cur = sums[li];
                let err = if tgt > 0.0 { (cur - tgt).abs() / tgt } else { cur.abs() };
                if err > mpe {
                    mpe = err;
                }
            }
        }
        max_prop_err = mpe;

        if verbose {
            rprintln!("Iter {}: max prop err = {:.6e}", it, max_prop_err);
        }

        if max_prop_err.is_finite() && max_prop_err <= tol {
            converged = true;
            break 'outer;
        }
    }

    list![
        weights = Doubles::from_values(w.into_iter()),
        converged = converged,
        iterations = iterations,
        max_prop_err = max_prop_err,
        diagnostics = list![
            iteration = Doubles::from_values(diag_iter.into_iter()),
            margin_index = Doubles::from_values(diag_margin.into_iter()),
            level_index = Doubles::from_values(diag_level.into_iter()),
            target = Doubles::from_values(diag_target.into_iter()),
            current = Doubles::from_values(diag_current.into_iter()),
            prop_err = Doubles::from_values(diag_err.into_iter())
        ]
    ]
    .into()
}

extendr_module! {
    mod ipf;
    fn rake_ipf_rust;
}
