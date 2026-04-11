# ipf

`ipf` provides fast iterative proportional fitting (raking) for survey
weighting. The computational core is written in Rust for speed and
numerical stability. It supports multiple raking variables, automatic
variable selection, weight bounding, and comprehensive diagnostics.

## Installation

You can install the development version of ipf from
[GitHub](https://github.com/) with:

``` r
# install.packages("pak")
pak::pak("christopherkenny/ipf")
```

## Example

Rake a survey sample to match known population targets:

``` r
library(ipf)
data(anes24)

# Define population targets as named proportions
targets <- list(
  sex = c(Male = 0.48, Female = 0.52),
  race = c(
    White = 0.59,
    Black = 0.14,
    Hispanic = 0.19,
    Asian = 0.06,
    "Native American" = 0.01,
    Other = 0.01
  )
)

# Rake
result <- rake(anes24, targets)
result
#> 
#> ── Raking result (ipf)
#> Converged: Yes (539 iterations, max prop err = 1e-06)
#> Variables raked: "sex" and "race"
#> Design effect: 1.145 | Effective n: 844 / 966
#> Weight range: [0.161, 3.226] | Mean: 1.008 | SD: 0.384
```

Examine per-variable diagnostics with
[`summary()`](https://rdrr.io/r/base/summary.html):

``` r
summary(result)
#> 
#> ── Raking Summary (ipf)
#> ────────────────────────────────────────────────────────────────────────────────
#> ✔ Converged in 539 iterations (max prop err = 1e-06)
#> ℹ No base weights (uniform)
#> ℹ Selection: type = "nolim", method = "total"
#> ℹ Variables raked: "sex" and "race"
#> ── Weight Summary ──────────────────────────────────────────────────────────────
#> Min: 0.1606 Q1: 0.8977 Median: 0.9024 Mean: 1.008 Q3: 1.1396 Max: 3.2257
#> SD: 0.3836 CV: 0.3805
#> ── Design Effect ───────────────────────────────────────────────────────────────
#> Deff: 1.1448 | Effective n: 844 / 966
#> ── Per-Variable Assessment ─────────────────────────────────────────────────────
#> 
#> ── sex
#> # A tibble: 3 × 9
#>   level  target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>   <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 Male     0.48          458          0.477       464.        0.480    0.00341
#> 2 Female   0.52          503          0.523       502.        0.520   -0.00341
#> 3 Total    1             961          1           966.        1.000    0.00683
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
#> 
#> ── race
#> # A tibble: 7 × 9
#>   level    target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>     <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 White      0.59          632        0.662       570.        0.590     -0.0718 
#> 2 Black      0.14          118        0.124       135.        0.140      0.0164 
#> 3 Hispanic   0.19          114        0.119       184.        0.190      0.0706 
#> 4 Asian      0.06           28        0.0293       58.0       0.0600     0.0307 
#> 5 Native …   0.01            3        0.00314       9.66      0.01000    0.00686
#> 6 Other      0.01           60        0.0628        9.66      0.01000   -0.0528 
#> 7 Total      1             955        1           966.        1.000      0.249  
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
```

Extract weights for downstream analysis:

``` r
augmented <- augment(result)
augmented[, c("sex", "race", ".weight")]
#> # A tibble: 966 × 3
#>    sex    race     .weight
#>    <chr>  <chr>      <dbl>
#>  1 Female White      0.898
#>  2 Male   White      0.902
#>  3 Female White      0.898
#>  4 Male   <NA>       0.706
#>  5 Female Black      1.14 
#>  6 Male   White      0.902
#>  7 Male   Black      1.15 
#>  8 Male   Hispanic   1.61 
#>  9 Male   White      0.902
#> 10 Female White      0.898
#> # ℹ 956 more rows
```

Get a tidy one-row-per-level view:

``` r
tidy(result)
#> # A tibble: 8 × 5
#>   variable level           target weighted_pct discrepancy
#>   <chr>    <chr>            <dbl>        <dbl>       <dbl>
#> 1 sex      Male              0.48      0.480     -1.88e- 9
#> 2 sex      Female            0.52      0.520      1.88e- 9
#> 3 race     White             0.59      0.590      1.89e-15
#> 4 race     Black             0.14      0.140      1.39e-15
#> 5 race     Hispanic          0.19      0.190      1.78e-15
#> 6 race     Asian             0.06      0.0600     5.48e-16
#> 7 race     Native American   0.01      0.01000    9.71e-17
#> 8 race     Other             0.01      0.01000    8.85e-17
```

Or a single-row model summary:

``` r
glance(result)
#> # A tibble: 1 × 7
#>   converged iterations max_prop_err  deff n_eff n_obs n_vars
#>   <lgl>          <int>        <dbl> <dbl> <dbl> <int>  <int>
#> 1 TRUE             539  0.000001000  1.14  844.   966      2
```
