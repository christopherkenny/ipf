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
#> Warning: Variable `race`: target levels "Native American" have no observations in data
#> (zero-cell).
result
#> 
#> ── Raking result (ipf)
#> Converged: No (1000 iterations, max prop err = 1)
#> Variables raked: "sex" and "race"
#> Missing handling: "ignore"
#> Design effect: 1.133 | Effective n: 853 / 966
#> Weight range: [0.152, 2.082] | Mean: 1.004 | SD: 0.366
```

Examine per-variable diagnostics with
[`summary()`](https://rdrr.io/r/base/summary.html):

``` r
summary(result)
#> Warning: Variable `race`: target levels "Native American" have no observations in data
#> (zero-cell).
#> 
#> ── Raking Summary (ipf)
#> ────────────────────────────────────────────────────────────────────────────────
#> ✖ Did NOT converge after 1000 iterations (max prop err = 1)
#> ℹ No base weights (uniform)
#> ℹ Selection: type = "nolim", method = "total"
#> ℹ Missing handling: "ignore"
#> ℹ Variables raked: "sex" and "race"
#> ── Weight Summary ──────────────────────────────────────────────────────────────
#> Min: 0.1519 Q1: 0.8946 Median: 0.9112 Mean: 1.0044 Q3: 1.1395 Max: 2.0822
#> SD: 0.366 CV: 0.3644
#> ── Design Effect ───────────────────────────────────────────────────────────────
#> Deff: 1.1328 | Effective n: 853 / 966
#> ── Per-Variable Assessment ─────────────────────────────────────────────────────
#> 
#> ── sex
#> # A tibble: 3 × 9
#>   level  target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>   <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 Male     0.48          458          0.477       464.        0.480    0.00341
#> 2 Female   0.52          503          0.523       502.        0.520   -0.00341
#> 3 Total    1             961          1           966.        1.00     0.00683
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
#> 
#> ── race
#> # A tibble: 7 × 9
#>   level    target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>     <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 White      0.59          632         0.662      570.         0.596     -0.0658
#> 2 Black      0.14          118         0.124      135.         0.141      0.0179
#> 3 Hispanic   0.19          114         0.119      184.         0.192      0.0725
#> 4 Asian      0.06           28         0.0293      58.0        0.0606     0.0313
#> 5 Native …   0.01            0         0            0          0          0     
#> 6 Other      0.01           63         0.0660       9.66       0.0101    -0.0559
#> 7 Total      1             955         1          956.         1.00       0.243 
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
```

Extract weights for downstream analysis:

``` r
augmented <- augment(result)
augmented[, c("sex", "race", ".weight")]
#> # A tibble: 966 × 3
#>    sex    race     .weight
#>    <chr>  <chr>      <dbl>
#>  1 Female White      0.895
#>  2 Male   White      0.911
#>  3 Female White      0.895
#>  4 Male   <NA>       1.29 
#>  5 Female Black      1.14 
#>  6 Male   White      0.911
#>  7 Male   Black      1.16 
#>  8 Male   Hispanic   1.63 
#>  9 Male   White      0.911
#> 10 Female White      0.895
#> # ℹ 956 more rows
```

Get a tidy one-row-per-level view:

``` r
tidy(result)
#> Warning: Variable `race`: target levels "Native American" have no observations in data
#> (zero-cell).
#> # A tibble: 8 × 5
#>   variable level           target weighted_pct discrepancy
#>   <chr>    <chr>            <dbl>        <dbl>       <dbl>
#> 1 sex      Male              0.48       0.480     7.33e-14
#> 2 sex      Female            0.52       0.520    -9.15e-14
#> 3 race     White             0.59       0.596    -5.96e- 3
#> 4 race     Black             0.14       0.141    -1.41e- 3
#> 5 race     Hispanic          0.19       0.192    -1.92e- 3
#> 6 race     Asian             0.06       0.0606   -6.06e- 4
#> 7 race     Native American   0.01       0         1   e- 2
#> 8 race     Other             0.01       0.0101   -1.01e- 4
```

Or a single-row model summary:

``` r
glance(result)
#> # A tibble: 1 × 7
#>   converged iterations max_prop_err  deff n_eff n_obs n_vars
#>   <lgl>          <int>        <dbl> <dbl> <dbl> <int>  <int>
#> 1 FALSE           1000            1  1.13  853.   966      2
```
