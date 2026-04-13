# Find discrepant variables and their aggregate discrepancy scores

Calculates discrepancy between the current weighted distribution and
target distributions for each variable, then aggregates using the chosen
method.

## Usage

``` r
find_discrepant_vars(
  data,
  targets,
  weights,
  choosemethod = "total",
  na_method = c("ignore", "bucket")
)
```

## Arguments

- data:

  Data frame.

- targets:

  Named list of named numeric target vectors (proportions).

- weights:

  Numeric weight vector.

- choosemethod:

  Method for aggregating per-category discrepancies. One of `"total"`,
  `"max"`, `"average"`, `"totalsquared"`, `"maxsquared"`,
  `"averagesquared"`.

- na_method:

  How to handle `NA` values. `"ignore"` excludes them from that margin.
  `"bucket"` treats missing values as an implicit extra category.

## Value

Named numeric vector of aggregate discrepancy per variable.

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
find_discrepant_vars(data, targets, weights = rep(1, 100))
#> gender    age 
#>   0.00   0.16 
```
