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

## Examples

``` r
data <- data.frame(
  gender = sample(c('M', 'F'), 100, replace = TRUE, prob = c(0.6, 0.4))
)
targets <- list(gender = c(M = 0.5, F = 0.5))
result <- rake(data, targets)
summary(result)
#> 
#> ── Raking Summary (ipf) 
#> ────────────────────────────────────────────────────────────────────────────────
#> ✔ Converged in 1 iterations (max prop err = 7.11e-16)
#> ℹ No base weights (uniform)
#> ℹ Selection: type = "nolim", method = "total"
#> ℹ Missing handling: "ignore"
#> ℹ Variables raked: "gender"
#> ── Weight Summary ──────────────────────────────────────────────────────────────
#> Min: 0.8333 Q1: 0.8333 Median: 0.8333 Mean: 1 Q3: 1.25 Max: 1.25
#> SD: 0.2041 CV: 0.2041
#> ── Design Effect ───────────────────────────────────────────────────────────────
#> Deff: 1.0417 | Effective n: 96 / 100
#> ── Per-Variable Assessment ─────────────────────────────────────────────────────
#> 
#> ── gender 
#> # A tibble: 3 × 9
#>   level target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>  <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 M        0.5           60            0.6         50        0.500    -0.1000
#> 2 F        0.5           40            0.4         50        0.500     0.100 
#> 3 Total    1            100            1          100        1.00      0.200 
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
#> 
```
