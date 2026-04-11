# Assess weight quality with diagnostic tables

Produces a per-variable diagnostic table comparing target distributions
to unweighted and weighted distributions.

## Usage

``` r
weight_assess(data, targets, weights, base_weights = NULL)
```

## Arguments

- data:

  Data frame.

- targets:

  Named list of named numeric target vectors (proportions).

- weights:

  Final raked weight vector.

- base_weights:

  Original base weights before raking. If `NULL`, uses uniform weights.

## Value

Named list of tibbles, one per variable.
