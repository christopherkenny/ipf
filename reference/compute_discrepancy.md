# Compute discrepancy between weighted distribution and targets

Compute discrepancy between weighted distribution and targets

## Usage

``` r
compute_discrepancy(data, targets, weights, var_name = NULL)
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

## Value

Named list with `weighted_pct` and `discrepancy` vectors per variable.
