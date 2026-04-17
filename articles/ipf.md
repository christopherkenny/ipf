# Getting Started with ipf

## Overview

Survey samples rarely match the population perfectly on key
demographics. **Raking** (iterative proportional fitting) adjusts case
weights so that the weighted sample margins match known population
targets. `ipf` makes this fast by using Rust to perform all
computations.

This vignette walks through a complete raking workflow using the bundled
`anes24` dataset, which is taken from a subset of the 2024 American
National Election Study
<https://electionstudies.org/data-center/2024-time-series-study/>.

## Setup

``` r
library(ipf)
library(tibble)
data(anes24)
```

The data contains 966 respondents from the ANES 2024 face-to-face
sample:

``` r
anes24
#> # A tibble: 966 × 7
#>    state sex    race     income     education    married  presidential
#>    <chr> <chr>  <chr>    <chr>      <chr>        <chr>    <chr>       
#>  1 CO    Female White    Over $100k NA           Divorced Trump       
#>  2 CA    Male   White    Over $100k NA           Married  Harris      
#>  3 NC    Female White    NA         Bachelor's   Married  Harris      
#>  4 MA    Male   NA       Over $100k Some college Married  Trump       
#>  5 WA    Female Black    NA         Bachelor's   Married  NA          
#>  6 GA    Male   White    $50k-$100k NA           NA       Trump       
#>  7 AR    Male   Black    Under $50k NA           NA       Harris      
#>  8 TX    Male   Hispanic Under $50k Some college NA       Trump       
#>  9 OR    Male   White    Under $50k NA           Divorced NA          
#> 10 AL    Female White    Under $50k NA           Divorced NA          
#> # ℹ 956 more rows
```

## Inspect data

Before writing targets, inspect the levels in your sample and see where
values are missing:

``` r
table(anes24$sex, useNA = 'ifany')
#> 
#> Female   Male   <NA> 
#>    503    458      5
table(anes24$race, useNA = 'ifany')
#> 
#>    Asian    Black Hispanic    Other    White     <NA> 
#>       28      118      114       63      632       11
table(anes24$income, useNA = 'ifany')
#> 
#> $50k-$100k Over $100k Under $50k       <NA> 
#>        300        389        230         47
```

When you define targets, the target names must match the data values
exactly. By default, `NA` values are ignored for that variable during
raking. If you use `na_method = 'bucket'`, `ipf` treats missing values
as an implicit extra category and preserves their total weight while
rescaling the named targets to the remaining nonmissing weight mass.

## Define population targets

Targets are a named list of named numeric vectors. Each vector’s names
must match the levels in your data, and the values should be proportions
summing to 1.

``` r
targets <- list(
  sex = c(Male = 0.472, Female = 0.528),
  race = c(
    White = 0.706,
    Black = 0.121,
    Hispanic = 0.107,
    Asian = 0.047,
    Other = 0.019
  ),
  income = c(
    'Under $50k' = 0.151,
    '$50k-$100k' = 0.294,
    'Over $100k' = 0.555
  )
)
```

If your targets don’t sum to 1, `ipf` will normalize them automatically
with a warning.

## Rake

The main function is
[`rake()`](http://christophertkenny.com/ipf/reference/rake.md):

``` r
result <- rake(anes24, targets, cap = NULL)
result
#> 
#> ── Raking result (ipf)
#> Converged: Yes (5 iterations, max prop err = 3.04e-07)
#> Variables raked: "sex", "race", and "income"
#> Missing handling: "exclude"
#> Design effect: 1.113 | Effective n: 868 / 966
#> Weight range: [0.199, 1.928] | Mean: 1 | SD: 0.337
```

`result` is an `ipf_rake` object containing the weights and diagnostics.

If you want missing values in a raking variable to act like their own
implicit category, use `na_method = 'bucket'`:

``` r
bucketed <- rake(anes24, targets, cap = NULL, na_method = 'bucket')
bucketed
#> 
#> ── Raking result (ipf)
#> Converged: Yes (6 iterations, max prop err = 5.46e-07)
#> Variables raked: "sex", "race", and "income"
#> Missing handling: "bucket"
#> Design effect: 1.114 | Effective n: 867 / 966
#> Weight range: [0.199, 1.936] | Mean: 1 | SD: 0.337
```

If you already have design weights, pass them through `base_weights`:

``` r
base_w <- ifelse(anes24$sex == 'Female', 1.1, 0.9)
base_w[is.na(base_w)] <- 1

base_weighted <- rake(anes24, targets, base_weights = base_w, cap = NULL)
base_weighted
#> 
#> ── Raking result (ipf)
#> Converged: Yes (5 iterations, max prop err = 3.04e-07)
#> Variables raked: "sex", "race", and "income"
#> Missing handling: "exclude"
#> Design effect: 1.113 | Effective n: 868 / 966
#> Weight range: [0.199, 1.928] | Mean: 1 | SD: 0.337
```

## Inspect results

### Design effect

The design effect measures how much the weighting inflates variance. A
deff of 1.0 means no inflation (uniform weights). Higher values mean
less effective data.

``` r
design_effect(result$weights)
#> $deff
#> [1] 1.113407
#> 
#> $n_eff
#> [1] 867.6076
```

### Per-variable diagnostics

[`summary()`](https://rdrr.io/r/base/summary.html) shows a full
diagnostic report:

``` r
summary(result)
#> 
#> ── Raking Summary (ipf)
#> ────────────────────────────────────────────────────────────────────────────────
#> ✔ Converged in 5 iterations (max prop err = 3.04e-07)
#> ℹ No base weights (uniform)
#> ℹ Selection: type = "nolim", method = "total"
#> ℹ Missing handling: "exclude"
#> ℹ Variables raked: "sex", "race", and "income"
#> ── Weight Summary ──────────────────────────────────────────────────────────────
#> Min: 0.1993 Q1: 0.6688 Median: 0.9703 Mean: 1 Q3: 1.2794 Max: 1.9275
#> SD: 0.3368 CV: 0.3368
#> ── Design Effect ───────────────────────────────────────────────────────────────
#> Deff: 1.1134 | Effective n: 868 / 966
#> ── Per-Variable Assessment ─────────────────────────────────────────────────────
#> 
#> ── sex
#> # A tibble: 3 × 9
#>   level  target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>   <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 Male    0.472          458          0.477       454.        0.472   -0.00459
#> 2 Female  0.528          503          0.523       508.        0.528    0.00459
#> 3 Total   1              961          1           962.        1.00     0.00917
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
#> 
#> ── race
#> # A tibble: 6 × 9
#>   level    target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>     <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 White     0.706          632         0.662       674.        0.706     0.0442 
#> 2 Black     0.121          118         0.124       116.        0.121    -0.00256
#> 3 Hispanic  0.107          114         0.119       102.        0.107    -0.0124 
#> 4 Asian     0.047           28         0.0293       44.9       0.0470    0.0177 
#> 5 Other     0.019           63         0.0660       18.1       0.0190   -0.0470 
#> 6 Total     1              955         1           955.        1.00      0.124  
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
#> 
#> ── income
#> # A tibble: 4 × 9
#>   level    target unweighted_n unweighted_pct weighted_n weighted_pct change_pct
#>   <chr>     <dbl>        <dbl>          <dbl>      <dbl>        <dbl>      <dbl>
#> 1 Under $…  0.151          230          0.250       139.        0.151    -0.0993
#> 2 $50k-$1…  0.294          300          0.326       270.        0.294    -0.0324
#> 3 Over $1…  0.555          389          0.423       509.        0.555     0.132 
#> 4 Total     1              919          1           917.        1.00      0.263 
#> # ℹ 2 more variables: residual_disc <dbl>, original_disc <dbl>
```

The **residual discrepancy** column shows how close the weighted
distribution is to the target.

### Tidy output

For programmatic use, the `broom`-style methods return `tibble`s:

``` r
# One row per variable-level
tidy(result)
#> # A tibble: 10 × 5
#>    variable level      target weighted_pct discrepancy
#>    <chr>    <chr>       <dbl>        <dbl>       <dbl>
#>  1 sex      Male        0.472       0.472     1.43e- 7
#>  2 sex      Female      0.528       0.528    -1.43e- 7
#>  3 race     White       0.706       0.706    -3.36e- 9
#>  4 race     Black       0.121       0.121     6.71e- 9
#>  5 race     Hispanic    0.107       0.107    -2.10e- 9
#>  6 race     Asian       0.047       0.0470   -1.69e- 9
#>  7 race     Other       0.019       0.0190    4.44e-10
#>  8 income   Under $50k  0.151       0.151     3.05e-16
#>  9 income   $50k-$100k  0.294       0.294     1.11e-16
#> 10 income   Over $100k  0.555       0.555    -2.55e-15

# One-row summary
glance(result)
#> # A tibble: 1 × 7
#>   converged iterations max_prop_err  deff n_eff n_obs n_vars
#>   <lgl>          <int>        <dbl> <dbl> <dbl> <int>  <int>
#> 1 TRUE               5  0.000000304  1.11  868.   966      3
```

### Augmenting the data

To use the weights in downstream analyses, attach them to your data:

``` r
weighted_data <- augment(result)
head(weighted_data)
#> # A tibble: 6 × 8
#>   state sex    race  income     education    married  presidential .weight
#>   <chr> <chr>  <chr> <chr>      <chr>        <chr>    <chr>          <dbl>
#> 1 CO    Female White Over $100k NA           Divorced Trump          1.37 
#> 2 CA    Male   White Over $100k NA           Married  Harris         1.28 
#> 3 NC    Female White NA         Bachelor's   Married  Harris         1.07 
#> 4 MA    Male   NA    Over $100k Some college Married  Trump          1.23 
#> 5 WA    Female Black NA         Bachelor's   Married  NA             1.12 
#> 6 GA    Male   White $50k-$100k NA           NA       Trump          0.908
```

The `.weight` column can then be used in downstream analyses.

For example, you can compare an estimate before and after weighting:

``` r
presidential_data <- subset(weighted_data, !is.na(presidential))

presidential_unweighted <- prop.table(table(presidential_data$presidential))

presidential_weighted <- aggregate(
  .weight ~ presidential,
  presidential_data,
  sum
)
presidential_weighted$weighted_pct <- presidential_weighted$.weight /
  sum(presidential_weighted$.weight)

presidential_compare <- tibble::tibble(
  presidential = presidential_weighted$presidential,
  unweighted_pct = as.numeric(presidential_unweighted[
    presidential_weighted$presidential
  ]),
  weighted_pct = presidential_weighted$weighted_pct
)

presidential_compare
#> # A tibble: 2 × 3
#>   presidential unweighted_pct weighted_pct
#>   <chr>                 <dbl>        <dbl>
#> 1 Harris                0.564        0.575
#> 2 Trump                 0.436        0.425
```

## Advanced options

### Weight bounding

By default, weights are capped at 5. Tighter bounds reduce extreme
weights but can leave more residual mismatch.

``` r
# Unbounded fit from above
range(result$weights)
#> [1] 0.1993456 1.9275117
design_effect(result$weights)
#> $deff
#> [1] 1.113407
#> 
#> $n_eff
#> [1] 867.6076

# Default cap
default_bounded <- rake(anes24, targets)
range(default_bounded$weights)
#> [1] 0.1993456 1.9275117
design_effect(default_bounded$weights)
#> $deff
#> [1] 1.113407
#> 
#> $n_eff
#> [1] 867.6076

# Tighter cap
tight <- rake(anes24, targets, cap = 3)
range(tight$weights)
#> [1] 0.1993456 1.9275117
design_effect(tight$weights)
#> $deff
#> [1] 1.113407
#> 
#> $n_eff
#> [1] 867.6076

# Or specify both min and max bounds
bounded <- rake(anes24, targets, bounds = c(0.3, 3))
range(bounded$weights)
#> [1] 0.300000 1.950523
```

### Variable selection

With many potential raking variables, you can let `ipf` select only the
most discrepant ones:

``` r
targets_many <- list(
  sex = c(Male = 0.472, Female = 0.528),
  race = c(
    White = 0.706,
    Black = 0.121,
    Hispanic = 0.107,
    Asian = 0.047,
    Other = 0.019
  ),
  income = c('Under $50k' = 0.151, '$50k-$100k' = 0.294, 'Over $100k' = 0.555),
  married = c(
    Married = 0.58,
    Widowed = 0.06,
    Divorced = 0.10,
    Separated = 0.01,
    'Never married' = 0.25
  )
)

# Only rake on variables where discrepancy exceeds 5%
result_pct <- rake(anes24, targets_many, type = 'pctlim', pctlim = 0.05)
result_pct$vars_used
#> [1] "race"    "income"  "married" "sex"
```

Use `type = 'nlim'` to select the top N most discrepant variables, or
`iterate = TRUE` to re-check after each round and add newly discrepant
variables.

### Checking discrepancies directly

You can inspect raw discrepancy scores without raking:

``` r
find_discrepant_vars(
  anes24,
  targets_many,
  weights = rep(1, nrow(anes24)),
  choosemethod = 'total'
)
#>         sex        race      income     married 
#> 0.009173777 0.123801047 0.263427639 0.483570392
```
