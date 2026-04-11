use extendr_api::prelude::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

/// One margin encoded with integer levels (0 = ignore/NA) and target totals.
#[derive(Clone)]
struct Margin {
    levels: Vec<usize>,
    targets: Vec<f64>,
}

fn list_get(list: &List, name: &str) -> Option<Robj> {
    for (k, v) in list.iter() {
        if k == name {
            return Some(v);
        }
    }
    None
}

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

/// Compute group sums for a single margin into `sums` (caller must zero first).
#[inline]
fn group_sums(m: &Margin, w: &[f64], sums: &mut [f64]) {
    let lcount = m.targets.len();
    for s in &mut sums[..lcount] {
        *s = 0.0;
    }
    for (i, &code) in m.levels.iter().enumerate() {
        if code > 0 && code <= lcount {
            sums[code - 1] += w[i];
        }
    }
}

fn record_diag_snapshot(
    ms: &[Margin],
    sums: &mut [f64],
    w: &[f64],
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
        group_sums(m, w, sums);
        for li in 0..lcount {
            let tgt = m.targets[li];
            let cur = sums[li];
            let err = if tgt > 0.0 {
                (cur - tgt).abs() / tgt
            } else {
                cur.abs()
            };
            diag_iter.push(iter_idx as f64);
            diag_margin.push((mi + 1) as f64);
            diag_level.push((li + 1) as f64);
            diag_target.push(tgt);
            diag_current.push(cur);
            diag_err.push(err);
        }
    }
}

// ---------------------------------------------------------------------------
// Core raking engine
// ---------------------------------------------------------------------------

/// Raking / iterative proportional fitting core.
///
/// @param weights Numeric vector (length n) of initial weights.
/// @param margins R list of margins, each with `$levels` (integer) and `$targets` (numeric).
/// @param max_iter Maximum number of full sweeps.
/// @param tol Convergence tolerance on max proportional error.
/// @param bounds NULL or numeric(2) c(lo, hi) for weight bounding.
/// @param grand_total Total weight mass to preserve after bounding.
/// @param diagnostics_every Record diagnostics every k iterations (0 = baseline only).
/// @param verbose Print iteration progress.
/// @keywords internal
/// @noRd
#[extendr]
fn rake_ipf_rust(
    weights: Doubles,
    margins: List,
    max_iter: i32,
    tol: f64,
    bounds: Robj,
    grand_total: f64,
    diagnostics_every: i32,
    verbose: bool,
) -> List {
    let mut w: Vec<f64> = weights
        .iter()
        .map(|x| if x.is_na() { 0.0 } else { x.inner() })
        .collect();
    let n = w.len();

    let bounds_opt: Option<(f64, f64)> = if bounds.is_null() {
        None
    } else {
        match TryInto::<Doubles>::try_into(bounds) {
            Ok(v) if v.len() == 2 => {
                let lo = if v[0].is_na() { f64::NEG_INFINITY } else { v[0].inner() };
                let hi = if v[1].is_na() { f64::INFINITY } else { v[1].inner() };
                if lo <= hi {
                    Some((lo, hi))
                } else {
                    rprintln!("Warning: bounds lo > hi; ignoring.");
                    None
                }
            }
            _ => {
                rprintln!("Warning: bounds must be numeric(2); ignoring.");
                None
            }
        }
    };

    let ms = parse_margins(margins).unwrap();
    let max_l = ms.iter().map(|m| m.targets.len()).max().unwrap_or(0);
    let mut sums = vec![0.0f64; max_l];
    let mut factors = vec![1.0f64; max_l];

    // Diagnostics storage
    let mut diag_iter = Vec::<f64>::new();
    let mut diag_margin = Vec::<f64>::new();
    let mut diag_level = Vec::<f64>::new();
    let mut diag_target = Vec::<f64>::new();
    let mut diag_current = Vec::<f64>::new();
    let mut diag_err = Vec::<f64>::new();

    // Save pre-raking weights
    let prevec: Vec<f64> = w.clone();

    // Baseline snapshot
    record_diag_snapshot(
        &ms,
        &mut sums,
        &w,
        0,
        &mut diag_iter,
        &mut diag_margin,
        &mut diag_level,
        &mut diag_target,
        &mut diag_current,
        &mut diag_err,
    );

    let mut converged = false;
    let mut iterations = 0i32;
    let mut max_prop_err = f64::INFINITY;

    for it in 1..=max_iter {
        iterations = it;

        // Sweep all margins
        for m in &ms {
            let lcount = m.targets.len();
            if lcount == 0 {
                continue;
            }
            group_sums(m, &w, &mut sums);
            for li in 0..lcount {
                factors[li] = if sums[li] > 0.0 {
                    m.targets[li] / sums[li]
                } else {
                    1.0
                };
            }
            for i in 0..n {
                let code = m.levels[i];
                if code > 0 && code <= lcount {
                    w[i] *= factors[code - 1];
                }
            }
        }

        // Post-sweep bounding (anesrake-style: cap then renormalize, repeat)
        if let Some((lo, hi)) = bounds_opt {
            let lo_finite = lo.is_finite();
            let hi_finite = hi.is_finite();
            for _ in 0..100 {
                let mut clamped = false;
                for wi in w.iter_mut() {
                    if hi_finite && *wi > hi + 1e-10 {
                        *wi = hi;
                        clamped = true;
                    }
                    if lo_finite && *wi < lo - 1e-10 {
                        *wi = lo;
                        clamped = true;
                    }
                }
                if !clamped {
                    break;
                }
                let s = kahan_sum(&w);
                if s > 0.0 {
                    let c = grand_total / s;
                    for wi in w.iter_mut() {
                        *wi *= c;
                    }
                }
            }
        }

        // Record diagnostics
        if diagnostics_every > 0 && (it % diagnostics_every == 0) {
            record_diag_snapshot(
                &ms,
                &mut sums,
                &w,
                it,
                &mut diag_iter,
                &mut diag_margin,
                &mut diag_level,
                &mut diag_target,
                &mut diag_current,
                &mut diag_err,
            );
        }

        // Convergence check: max proportional error across all margin-levels
        let mut mpe = 0.0f64;
        for m in &ms {
            let lcount = m.targets.len();
            if lcount == 0 {
                continue;
            }
            group_sums(m, &w, &mut sums);
            for li in 0..lcount {
                let tgt = m.targets[li];
                let cur = sums[li];
                let err = if tgt > 0.0 {
                    (cur - tgt).abs() / tgt
                } else {
                    cur.abs()
                };
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
            break;
        }
    }

    list!(
        weights = Doubles::from_values(w.into_iter()),
        prevec = Doubles::from_values(prevec.into_iter()),
        converged = converged,
        iterations = iterations,
        max_prop_err = max_prop_err,
        diagnostics = list!(
            iteration = Doubles::from_values(diag_iter.into_iter()),
            margin_index = Doubles::from_values(diag_margin.into_iter()),
            level_index = Doubles::from_values(diag_level.into_iter()),
            target = Doubles::from_values(diag_target.into_iter()),
            current = Doubles::from_values(diag_current.into_iter()),
            prop_err = Doubles::from_values(diag_err.into_iter()),
        ),
    )
    .into()
}

// ---------------------------------------------------------------------------
// Discrepancy computation
// ---------------------------------------------------------------------------

/// Compute weighted proportions and discrepancy from targets for a single variable.
///
/// @param weights Numeric weight vector.
/// @param levels Integer-coded variable (0 = NA/ignore, 1..L = categories).
/// @param targets Numeric target proportions (length L, should sum to 1).
/// @keywords internal
/// @noRd
#[extendr]
fn compute_discrepancy_rust(weights: Doubles, levels: Integers, targets: Doubles) -> List {
    let n = weights.len();
    let l = targets.len();
    let mut sums = vec![0.0f64; l];
    let mut total = 0.0f64;

    for i in 0..n {
        let wi = if weights[i].is_na() {
            0.0
        } else {
            weights[i].inner()
        };
        let code = if levels[i].is_na() {
            0
        } else {
            levels[i].inner().max(0) as usize
        };
        if code > 0 && code <= l {
            sums[code - 1] += wi;
            total += wi;
        }
    }

    let mut wpct = vec![0.0f64; l];
    let mut disc = vec![0.0f64; l];
    for li in 0..l {
        let tgt = if targets[li].is_na() {
            0.0
        } else {
            targets[li].inner()
        };
        wpct[li] = if total > 0.0 { sums[li] / total } else { 0.0 };
        disc[li] = tgt - wpct[li];
    }

    list!(
        weighted_pct = Doubles::from_values(wpct.into_iter()),
        discrepancy = Doubles::from_values(disc.into_iter()),
    )
    .into()
}

// ---------------------------------------------------------------------------
// Design effect
// ---------------------------------------------------------------------------

/// Compute design effect and effective sample size from a weight vector.
///
/// @param weights Numeric weight vector.
/// @keywords internal
/// @noRd
#[extendr]
fn design_effect_rust(weights: Doubles) -> List {
    let w: Vec<f64> = weights
        .iter()
        .filter(|x| !x.is_na())
        .map(|x| x.inner())
        .collect();
    let n = w.len() as f64;
    if n == 0.0 {
        return list!(deff = f64::NAN, n_eff = f64::NAN).into();
    }
    let sum_w = kahan_sum(&w);
    if sum_w == 0.0 {
        return list!(deff = f64::NAN, n_eff = f64::NAN).into();
    }
    // Rescale to mean 1
    let scale = n / sum_w;
    let sum_w2: f64 = w.iter().map(|&x| {
        let xs = x * scale;
        xs * xs
    }).sum();
    let deff = sum_w2 / n;
    let n_eff = n / deff;
    list!(deff = deff, n_eff = n_eff).into()
}

// ---------------------------------------------------------------------------
// Weight summary statistics
// ---------------------------------------------------------------------------

/// Compute summary statistics for a weight vector.
///
/// @param weights Numeric weight vector.
/// @keywords internal
/// @noRd
#[extendr]
fn weight_summary_rust(weights: Doubles) -> List {
    let mut w: Vec<f64> = weights
        .iter()
        .filter(|x| !x.is_na())
        .map(|x| x.inner())
        .collect();
    let n = w.len();
    if n == 0 {
        return list!(
            min = f64::NAN,
            q1 = f64::NAN,
            median = f64::NAN,
            mean = f64::NAN,
            q3 = f64::NAN,
            max = f64::NAN,
            sd = f64::NAN,
            cv = f64::NAN,
        )
        .into();
    }
    w.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = w[0];
    let max = w[n - 1];
    let mean = kahan_sum(&w) / n as f64;

    let quantile = |p: f64| -> f64 {
        let h = p * (n as f64 - 1.0);
        let lo = h.floor() as usize;
        let hi = lo + 1;
        let frac = h - lo as f64;
        if hi >= n {
            w[n - 1]
        } else {
            w[lo] * (1.0 - frac) + w[hi] * frac
        }
    };
    let q1 = quantile(0.25);
    let median = quantile(0.5);
    let q3 = quantile(0.75);

    let var: f64 = w.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>() / n as f64;
    let sd = var.sqrt();
    let cv = if mean > 0.0 { sd / mean } else { f64::NAN };

    list!(
        min = min,
        q1 = q1,
        median = median,
        mean = mean,
        q3 = q3,
        max = max,
        sd = sd,
        cv = cv,
    )
    .into()
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

extendr_module! {
    mod ipf;
    fn rake_ipf_rust;
    fn compute_discrepancy_rust;
    fn design_effect_rust;
    fn weight_summary_rust;
}
