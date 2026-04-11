# Glance at an ipf_rake object

Returns a single-row tibble with summary statistics.

## Usage

``` r
# S3 method for class 'ipf_rake'
glance(x, ...)
```

## Arguments

- x:

  An `ipf_rake` object.

- ...:

  Additional arguments (ignored).

## Value

A single-row tibble with columns: `converged`, `iterations`,
`max_prop_err`, `deff`, `n_eff`, `n_obs`, `n_vars`.
