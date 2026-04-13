# Assess weight quality with diagnostic tables

Produces a per-variable diagnostic table comparing target distributions
to unweighted and weighted distributions.

## Usage

``` r
weight_assess(
  data,
  targets,
  weights,
  base_weights = NULL,
  na_method = c("ignore", "bucket")
)
```

## Arguments

- data:

  Data frame.

- targets:

  Named list of named numeric target vectors (proportions).

- weights:

  Final raked weight vector.

- base_weights:

  Original base weights before raking. If `NULL`, uses uniform weights.

- na_method:

  How to handle `NA` values. `"ignore"` excludes them from that margin.
  `"bucket"` treats missing values as an implicit extra category.

## Value

Named list of tibbles, one per variable.

## Examples

``` r
data <- data.frame(
  gender = sample(c('M', 'F'), 100, replace = TRUE, prob = c(0.6, 0.4))
)
targets <- list(gender = c(M = 0.5, F = 0.5))
result <- rake(data, targets)
weight_assess(data, targets, result$weights)
#> $gender
#> # A tibble: 3 × 9
#>   level target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>  <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 M        0.5           59           0.59         50          0.5      -0.09
#> 2 F        0.5           41           0.41         50          0.5       0.09
#> 3 Total    1            100           1           100          1         0.18
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
#> 
```
