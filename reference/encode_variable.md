# Encode a variable to integer codes for the Rust raking engine

Converts factor, character, logical, or integer variables to 1-based
integer codes. NAs either become 0 (ignored by the Rust core) or an
explicit missing bucket.

## Usage

``` r
encode_variable(
  x,
  target_names,
  var_name = "variable",
  na_method = c("ignore", "bucket")
)
```

## Arguments

- x:

  A vector from the data frame.

- target_names:

  Character vector of expected level names from the target.

- var_name:

  Name of the variable (for error messages).

- na_method:

  How to handle `NA` values. `"ignore"` excludes them from that margin.
  `"bucket"` treats missing values as an implicit extra category.

## Value

A list with:

- `codes`: integer vector (0 = NA, 1..L = categories)

- `level_names`: character vector of level names in order
