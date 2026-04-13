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

## Examples

``` r
w <- c(1.2, 0.8, 1.5, 0.5, 1.0)
design_effect(w)
#> $deff
#> [1] 1.116
#> 
#> $n_eff
#> [1] 4.480287
#> 
```
