make_margins_list <- function(m, w) {
  grand_total <- sum(w)
  stopifnot(all(c('var', 'targets') %in% names(m)))
  v <- m$var
  tg <- m$targets
  vals <- df[[v]]
  if (!is.factor(vals)) {
    vals <- factor(vals)
  }
  stopifnot(is.numeric(tg), !is.null(names(tg)))
  s <- sum(tg, na.rm = TRUE)

  if (abs(s - 1) < 1e-8) {
    tg_tot <- tg * grand_total
  } else {
    tg_tot <- tg * (grand_total / s)
  }
  lev <- union(levels(vals), names(tg_tot))
  vals <- factor(vals, levels = lev)
  codes <- as.integer(vals)
  codes[is.na(codes)] <- 0L
  tg_vec <- as.numeric(tg_tot[lev])
  cur <- tapply(w, vals, sum, default = 0)
  zero_cells <- lev[which(cur == 0 & tg_vec > 0)]
  if (length(zero_cells)) {
    warning(sprintf('`%s`: zero-weight sample for %s', v, paste(zero_cells, collapse = ', ')))
  }

  list(
    name = v,
    level_names = lev,
    levels = codes,
    targets = tg_vec
  )
}

prep_margins_for_rust <- function(data, weight_col, margins) {
  weight_col <- rlang::ensym(weight_col)
  df <- tibble::as_tibble(data)
  w <- df[[rlang::as_string(weight_col)]]
  stopifnot(is.numeric(w), all(is.finite(w)), all(w > 0))

  l_margins <- lapply(margins, make_margins_list)

  list(
    weights = w,
    grand_total = sum(w),
    margins = l_margins
  )
}

rake_weights <- function(data, weight_col, margins,
                         max_iter = 50, tol = 1e-6,
                         bounds = NULL, diagnostics_every = 1L,
                         verbose = FALSE) {
  prep <- prep_margins_for_rust(data, {{ weight_col }}, margins)
  res <- rake_ipf_rust(
    weights = prep$weights,
    margins = prep$margins,
    max_iter = as.integer(max_iter),
    tol = tol,
    bounds = if (is.null(bounds)) NULL else as.numeric(bounds),
    grand_total = prep$grand_total,
    diagnostics_every = as.integer(diagnostics_every),
    verbose = isTRUE(verbose)
  )
  out_weights <- res$weights
  diag <- tibble::as_tibble(res$diagnostics)
  if (!is.null(diag$margin_index) && !is.null(res$margin_level_names)) {
    mapper <- function(mi, li) res$margin_level_names[[mi]][li]
    diag$level <- purrr::map2_chr(diag$margin_index, diag$level_index, mapper)
  }

  list(
    data = dplyr::mutate(tibble::as_tibble(data), .weight_raked = out_weights),
    converged = isTRUE(res$converged),
    iterations = as.integer(res$iterations),
    diagnostics = diag
  )
}
