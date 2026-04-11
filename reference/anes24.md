# ANES 2024 Time Series Study (subset)

A subset of the 2024 American National Election Study (ANES) Time Series
face-to-face sample, containing demographic and vote choice variables
for 966 respondents. Useful for demonstrating survey raking workflows.

## Usage

``` r
anes24
```

## Format

A tibble with 966 rows and 7 columns:

- state:

  Two-letter US state abbreviation. `NA` for respondents whose state is
  not identified (106 missing).

- sex:

  Respondent sex: `"Male"` or `"Female"` (5 missing).

- race:

  Race/ethnicity: `"White"`, `"Black"`, `"Hispanic"`, `"Asian"`, or
  `"Other"` (11 missing).

- income:

  Household income bracket: `"Under $50k"`, `"$50k-$100k"`, or
  `"Over $100k"` (47 missing).

- education:

  Education: `"Less than HS"`, `"High school"`, `"Some college"`,
  `"Bachelor's"`, or `"Graduate"` (451 missing).

- married:

  Marital status: `"Married"`, `"Widowed"`, `"Divorced"`, `"Separated"`,
  or `"Never married"` (277 missing).

- presidential:

  2024 presidential vote choice: `"Harris"` or `"Trump"` (335 missing).

## Source

<https://electionstudies.org/data-center/2024-time-series-study/>

## References

American National Election Studies. 2025. *ANES 2024 Time Series Study
Full Release* (dataset and documentation). August 8, 2025 version.
<https://www.electionstudies.org/>
