# Find discrepant variables and their aggregate discrepancy scores

Calculates discrepancy between the current weighted distribution and
target distributions for each variable, then aggregates using the chosen
method.

## Usage

``` r
find_discrepant_vars(data, targets, weights, choosemethod = "total")
```

## Arguments

- data:

  Data frame.

- targets:

  Named list of named numeric target vectors (proportions).

- weights:

  Numeric weight vector.

- choosemethod:

  Method for aggregating per-category discrepancies. One of `"total"`,
  `"max"`, `"average"`, `"totalsquared"`, `"maxsquared"`,
  `"averagesquared"`.

## Value

Named numeric vector of aggregate discrepancy per variable.
