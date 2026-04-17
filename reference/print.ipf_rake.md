# Print an ipf_rake object

Print an ipf_rake object

## Usage

``` r
# S3 method for class 'ipf_rake'
print(x, ...)
```

## Arguments

- x:

  An `ipf_rake` object.

- ...:

  Additional arguments (ignored).

## Value

Invisibly returns `x`.

## Examples

``` r
data <- data.frame(
  gender = sample(c('M', 'F'), 100, replace = TRUE, prob = c(0.6, 0.4))
)
targets <- list(gender = c(M = 0.5, F = 0.5))
result <- rake(data, targets)
print(result)
#> 
#> ── Raking result (ipf) 
#> Converged: Yes (1 iterations, max prop err = 4.44e-16)
#> Variables raked: "gender"
#> Missing handling: "exclude"
#> Design effect: 1.085 | Effective n: 92 / 100
#> Weight range: [0.781, 1.389] | Mean: 1 | SD: 0.292
```
