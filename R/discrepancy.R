#' Compute discrepancy between weighted distribution and targets
#'
#' @param data Data frame containing the variables.
#' @param targets Named list of named numeric target vectors (proportions).
#' @param weights Numeric weight vector.
#' @param var_name Single variable name to compute discrepancy for.
#'   If `NULL`, computes for all variables in `targets`.
#'
#' @return Named list with `weighted_pct` and `discrepancy` vectors per variable.
#'
#' @keywords internal
compute_discrepancy <- function(data, targets, weights, var_name = NULL) {
  vars <- if (is.null(var_name)) names(targets) else var_name

  results <- lapply(stats::setNames(vars, vars), function(v) {
    enc <- encode_variable(data[[v]], names(targets[[v]]), var_name = v)
    tgt_ordered <- targets[[v]][enc$level_names]
    tgt_ordered[is.na(tgt_ordered)] <- 0

    compute_discrepancy_rust(
      weights = weights,
      levels = enc$codes,
      targets = tgt_ordered
    )
  })

  results
}

#' Find discrepant variables and their aggregate discrepancy scores
#'
#' Calculates discrepancy between the current weighted distribution and target distributions for each variable, then aggregates using the chosen method.
#'
#' @param data Data frame.
#' @param targets Named list of named numeric target vectors (proportions).
#' @param weights Numeric weight vector.
#' @param choosemethod Method for aggregating per-category discrepancies.
#'   One of `"total"`, `"max"`, `"average"`, `"totalsquared"`, `"maxsquared"`, `"averagesquared"`.
#'
#' @return Named numeric vector of aggregate discrepancy per variable.
#'
#' @export
find_discrepant_vars <- function(
  data,
  targets,
  weights,
  choosemethod = 'total'
) {
  valid_methods <- c(
    'total',
    'max',
    'average',
    'totalsquared',
    'maxsquared',
    'averagesquared'
  )
  choosemethod <- match.arg(choosemethod, valid_methods)

  disc_list <- compute_discrepancy(data, targets, weights)

  vapply(
    disc_list,
    function(d) {
      disc <- d$discrepancy
      switch(choosemethod,
        total = sum(abs(disc)),
        max = max(abs(disc)),
        average = mean(abs(disc)),
        totalsquared = sum(disc^2),
        maxsquared = max(disc^2),
        averagesquared = mean(disc^2)
      )
    },
    numeric(1)
  )
}
