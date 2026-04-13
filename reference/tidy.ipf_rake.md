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

## Examples

``` r
data <- data.frame(
  gender = sample(c('M', 'F'), 100, replace = TRUE, prob = c(0.6, 0.4))
)
targets <- list(gender = c(M = 0.5, F = 0.5))
result <- rake(data, targets)
tidy(result)
#> # A tibble: 2 × 5
#>   variable level target weighted_pct discrepancy
#>   <chr>    <chr>  <dbl>        <dbl>       <dbl>
#> 1 gender   M        0.5        0.500    8.33e-16
#> 2 gender   F        0.5        0.500   -3.33e-16
```
