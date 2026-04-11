library(tidyverse)

anes24 <- read_csv('data-raw/anes_timeseries_2024_csv_20250808.csv') |>
  filter(V240002a == 1) |>
  select(
    state = V241023,
    sex = V241550,
    race = V241501x,
    income = V241566x,
    education = V241469x,
    married = V241459,
    presidential = V242096x
  )

# State FIPS to two-letter abbreviation
state_fips <- c(
  `1` = 'AL',
  `2` = 'AK',
  `4` = 'AZ',
  `5` = 'AR',
  `6` = 'CA',
  `8` = 'CO',
  `9` = 'CT',
  `10` = 'DE',
  `11` = 'DC',
  `12` = 'FL',
  `13` = 'GA',
  `15` = 'HI',
  `16` = 'ID',
  `17` = 'IL',
  `18` = 'IN',
  `19` = 'IA',
  `20` = 'KS',
  `21` = 'KY',
  `22` = 'LA',
  `23` = 'ME',
  `24` = 'MD',
  `25` = 'MA',
  `26` = 'MI',
  `27` = 'MN',
  `28` = 'MS',
  `29` = 'MO',
  `30` = 'MT',
  `31` = 'NE',
  `32` = 'NV',
  `33` = 'NH',
  `34` = 'NJ',
  `35` = 'NM',
  `36` = 'NY',
  `37` = 'NC',
  `38` = 'ND',
  `39` = 'OH',
  `40` = 'OK',
  `41` = 'OR',
  `42` = 'PA',
  `44` = 'RI',
  `45` = 'SC',
  `46` = 'SD',
  `47` = 'TN',
  `48` = 'TX',
  `49` = 'UT',
  `50` = 'VT',
  `51` = 'VA',
  `53` = 'WA',
  `54` = 'WV',
  `55` = 'WI',
  `56` = 'WY'
)

anes24 <- anes24 |>
  mutate(
    state = unname(state_fips[as.character(state)]),
    sex = case_when(
      sex == 1 ~ 'Male',
      sex == 2 ~ 'Female',
      .default = NA_character_
    ),
    race = case_when(
      race == 1 ~ 'White',
      race == 2 ~ 'Black',
      race == 3 ~ 'Hispanic',
      race == 4 ~ 'Asian',
      race %in% c(5, 6) ~ 'Other',
      .default = NA_character_
    ),
    income = case_when(
      income >= 1 & income <= 10 ~ 'Under $50k',
      income >= 11 & income <= 20 ~ '$50k-$100k',
      income >= 21 ~ 'Over $100k',
      .default = NA_character_
    ),
    education = case_when(
      education == 1 ~ 'Less than HS',
      education == 2 ~ 'High school',
      education == 3 ~ 'Some college',
      education == 4 ~ "Bachelor's",
      education == 5 ~ 'Graduate',
      .default = NA_character_
    ),
    married = case_when(
      married == 1 ~ 'Married',
      married == 2 ~ 'Widowed',
      married == 3 ~ 'Divorced',
      married == 4 ~ 'Separated',
      married == 5 ~ 'Never married',
      .default = NA_character_
    ),
    presidential = case_when(
      presidential == 1 ~ 'Harris',
      presidential == 2 ~ 'Trump',
      .default = NA_character_
    )
  )

anes24 <- as_tibble(anes24)

usethis::use_data(anes24, overwrite = TRUE)
