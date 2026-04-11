# Summarize an ipf_rake object

Produces a detailed summary including per-variable diagnostic tables
showing target vs. achieved distributions.

## Usage

``` r
# S3 method for class 'ipf_rake'
summary(object, ...)
```

## Arguments

- object:

  An `ipf_rake` object.

- ...:

  Additional arguments (ignored).

## Value

Invisibly returns a list with convergence info, weight summary, design
effect, and per-variable assessment tibbles.
