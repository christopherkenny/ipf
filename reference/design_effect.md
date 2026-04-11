# Compute design effect and effective sample size

The design effect (deff) measures the variance inflation factor due to
unequal weighting. The effective sample size is `n / deff`.

## Usage

``` r
design_effect(weights)
```

## Arguments

- weights:

  Numeric weight vector.

## Value

A list with `deff` (design effect) and `n_eff` (effective sample size).
