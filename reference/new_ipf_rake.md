# Construct an ipf_rake object

Construct an ipf_rake object

## Usage

``` r
new_ipf_rake(
  weights,
  data,
  converged,
  iterations,
  max_prop_err,
  targets,
  vars_used,
  base_weights,
  type,
  choosemethod,
  na_method,
  cap,
  deff,
  n_eff,
  diagnostics
)
```

## Arguments

- weights:

  Final raked weight vector.

- data:

  Original data frame.

- converged:

  Logical, whether convergence was achieved.

- iterations:

  Number of iterations.

- max_prop_err:

  Final maximum proportional error.

- targets:

  Normalized targets used.

- vars_used:

  Character vector of variables raked on.

- base_weights:

  Original base weights.

- type:

  Variable selection type.

- choosemethod:

  Discrepancy aggregation method.

- na_method:

  Missing-data handling method.

- cap:

  Weight cap value.

- deff:

  Design effect.

- n_eff:

  Effective sample size.

- diagnostics:

  Diagnostics tibble.

## Value

An `ipf_rake` S3 object.
