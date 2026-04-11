# Encode margins list for the Rust raking engine

Encode margins list for the Rust raking engine

## Usage

``` r
encode_margins_for_rust(data, targets, weights)
```

## Arguments

- data:

  Data frame.

- targets:

  Named list of named numeric targets (proportions, sum to 1).

- weights:

  Current weight vector.

## Value

List of margin lists, each with `$levels` and `$targets`.
