# Validate and normalize target distributions

Ensures targets are named numeric vectors that sum to 1 (proportions).
Normalizes them if they appear to be counts or percentages.

## Usage

``` r
encode_targets(targets, data)
```

## Arguments

- targets:

  Named list of named numeric vectors.

- data:

  Data frame to validate against.

## Value

Normalized targets list (each summing to 1).
