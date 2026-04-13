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

## Examples

``` r
data <- data.frame(
  gender = sample(c('M', 'F'), 100, replace = TRUE, prob = c(0.6, 0.4))
)
targets <- list(gender = c(M = 0.5, F = 0.5))
result <- rake(data, targets)
glance(result)
#> # A tibble: 1 × 7
#>   converged iterations max_prop_err  deff n_eff n_obs n_vars
#>   <lgl>          <int>        <dbl> <dbl> <dbl> <int>  <int>
#> 1 TRUE               1     1.14e-15  1.00  99.8   100      1
```
