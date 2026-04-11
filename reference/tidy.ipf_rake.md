# Tidy an ipf_rake object

Returns a one-row-per-variable-per-level tibble with target proportions,
weighted proportions, and discrepancy.

## Usage

``` r
# S3 method for class 'ipf_rake'
tidy(x, ...)
```

## Arguments

- x:

  An `ipf_rake` object.

- ...:

  Additional arguments (ignored).

## Value

A tibble with columns: `variable`, `level`, `target`, `weighted_pct`,
`discrepancy`.
