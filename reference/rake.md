# Iterative proportional fitting (raking)

Adjusts survey weights so that weighted marginal distributions match
known population targets. Supports automatic variable selection,
iterative re-raking, and weight bounding.

## Usage

``` r
rake(
  data,
  targets,
  base_weights = NULL,
  cap = 5,
  bounds = NULL,
  type = c("nolim", "pctlim", "nlim"),
  pctlim = 0.05,
  nlim = 5L,
  choosemethod = c("total", "max", "average", "totalsquared", "maxsquared",
    "averagesquared"),
  na_method = c("ignore", "bucket"),
  iterate = TRUE,
  max_iter = 1000L,
  tol = 1e-06,
  verbose = FALSE,
  diagnostics_every = 0L
)
```

## Arguments

- data:

  A data frame or tibble containing the survey data.

- targets:

  A named list of named numeric vectors specifying target proportions
  for each raking variable. Names of the list must match column names in
  `data`. Each vector's names must match the levels of the corresponding
  variable. Values should sum to 1 (proportions); if not, they are
  normalized with a warning.

- base_weights:

  Optional numeric vector of base (design) weights. If `NULL` (default),
  uniform weights of 1 are used. Centered to mean 1 before raking.

- cap:

  Maximum weight value (ratio cap). Weights exceeding this value are
  trimmed and all weights are renormalized. Default `5`. Ignored if
  `bounds` is specified.

- bounds:

  Optional numeric vector of length 2, `c(lo, hi)`, specifying minimum
  and maximum weight bounds. Overrides `cap`.

- type:

  Variable selection method:

  - `"nolim"` (default): use all variables in `targets`.

  - `"pctlim"`: use only variables with discrepancy \>= `pctlim`.

  - `"nlim"`: use the `nlim` most discrepant variables.

- pctlim:

  Discrepancy threshold for `type = "pctlim"`. Default `0.05` (5
  percentage points).

- nlim:

  Number of variables for `type = "nlim"`. Default `5`.

- choosemethod:

  Method for aggregating per-category discrepancies into a single
  variable score. One of `"total"`, `"max"`, `"average"`,
  `"totalsquared"`, `"maxsquared"`, `"averagesquared"`.

- na_method:

  How to handle `NA` values in raking variables. `"ignore"` excludes
  missing cases from that variable's margin update. `"bucket"` treats
  missing values as an implicit extra category whose total weight is
  preserved while the named targets are rescaled to the remaining
  nonmissing weight mass.

- iterate:

  Logical. If `TRUE` and `type = "pctlim"`, re-check discrepancies after
  raking and add newly discrepant variables, repeating up to 10 times.
  Default `TRUE`.

- max_iter:

  Maximum number of raking iterations. Default `1000`.

- tol:

  Convergence tolerance (max proportional error). Default `1e-6`.

- verbose:

  Logical. If `TRUE`, print iteration progress. Default `FALSE`.

- diagnostics_every:

  Record per-margin diagnostics every `k` iterations. `0` means only
  baseline. Default `0`.

## Value

An `ipf_rake` object (S3 class) containing:

- `weights`: final raked weight vector

- `data`: the input data frame

- `converged`: logical

- `iterations`: number of iterations

- `max_prop_err`: final max proportional error

- `targets`: normalized targets used

- `vars_used`: character vector of variables raked on

- `base_weights`: original base weights

- `type`, `choosemethod`, `na_method`, `cap`: settings used

- `deff`, `n_eff`: design effect and effective sample size

- `diagnostics`: tibble of per-iteration diagnostics

## Examples

``` r
data <- data.frame(
  gender = sample(c('M', 'F'), 100, replace = TRUE, prob = c(0.6, 0.4)),
  age = sample(c('young', 'old'), 100, replace = TRUE, prob = c(0.7, 0.3))
)
targets <- list(
  gender = c(M = 0.5, F = 0.5),
  age = c(young = 0.6, old = 0.4)
)
result <- rake(data, targets)
print(result)
#> 
#> ── Raking result (ipf) 
#> Converged: Yes (2 iterations, max prop err = 3.13e-08)
#> Variables raked: "gender" and "age"
#> Missing handling: "ignore"
#> Design effect: 1.074 | Effective n: 93 / 100
#> Weight range: [0.735, 1.55] | Mean: 1 | SD: 0.272
```
