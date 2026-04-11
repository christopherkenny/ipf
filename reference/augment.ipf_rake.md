# Augment data with raked weights

Returns the original data frame with a `.weight` column appended.

## Usage

``` r
# S3 method for class 'ipf_rake'
augment(x, ...)
```

## Arguments

- x:

  An `ipf_rake` object.

- ...:

  Additional arguments (ignored).

## Value

A tibble with all original columns plus `.weight`.
