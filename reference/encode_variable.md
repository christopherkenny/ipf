# Encode a variable to integer codes for the Rust raking engine

Converts factor, character, logical, or integer variables to 1-based
integer codes. NAs become 0 (ignored by the Rust core).

## Usage

``` r
encode_variable(x, target_names, var_name = "variable")
```

## Arguments

- x:

  A vector from the data frame.

- target_names:

  Character vector of expected level names from the target.

- var_name:

  Name of the variable (for error messages).

## Value

A list with:

- `codes`: integer vector (0 = NA, 1..L = categories)

- `level_names`: character vector of level names in order
