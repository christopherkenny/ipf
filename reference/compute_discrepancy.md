# Compute discrepancy between weighted distribution and targets

Compute discrepancy between weighted distribution and targets

## Usage

``` r
compute_discrepancy(
  data,
  targets,
  weights,
  var_name = NULL,
  na_method = c("ignore", "bucket")
)
```

## Arguments

- data:

  Data frame containing the variables.

- targets:

  Named list of named numeric target vectors (proportions).

- weights:

  Numeric weight vector.

- var_name:

  Single variable name to compute discrepancy for. If `NULL`, computes
  for all variables in `targets`.

- na_method:

  How to handle `NA` values. `"ignore"` excludes them from that margin.
  `"bucket"` treats missing values as an implicit extra category.

## Value

Named list with `weighted_pct` and `discrepancy` vectors per variable.
