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

## Examples

``` r
data <- data.frame(
  gender = sample(c('M', 'F'), 100, replace = TRUE, prob = c(0.6, 0.4))
)
targets <- list(gender = c(M = 0.5, F = 0.5))
result <- rake(data, targets)
augment(result)
#> # A tibble: 100 × 2
#>    gender .weight
#>    <chr>    <dbl>
#>  1 M        0.758
#>  2 F        1.47 
#>  3 F        1.47 
#>  4 M        0.758
#>  5 M        0.758
#>  6 M        0.758
#>  7 M        0.758
#>  8 M        0.758
#>  9 F        1.47 
#> 10 F        1.47 
#> # ℹ 90 more rows
```
