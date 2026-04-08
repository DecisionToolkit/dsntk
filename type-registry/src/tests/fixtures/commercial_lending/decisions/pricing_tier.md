---
dmn:
  id: pricing_tier
  type: decision
  name: Pricing Tier
  requires:
    - credit_score_classification
    - collateral_adequacy
    - dscr_calculation
    - sector_risk_assessment
    - accounts_quality_check
  governed-by:
    - commercial_lending_policy
---

# Pricing Tier

Determines the base pricing tier by combining credit grade, collateral
adequacy, debt service coverage, sector risk, and accounts quality.
Uses **PRIORITY** hit policy — first matching rule wins, so rules are
ordered from best tier to worst.

Any upstream "Decline" result propagates as a decline here. The DSCR
hard floor of 1.0 is also enforced.

## Decision table

> # Pricing Tier
> Pricing Tier

| P  | Credit Grade | Collateral Grade | DSCR     | Sector Risk | Accounts Quality | Pricing Tier                | Base Rate |
|:--:|:------------:|:----------------:|:--------:|:-----------:|:----------------:|:---------------------------:|:---------:|
|    |              |                  |          |             |                  | "T1","T2","T3","T4","T5","Decline" |    |
|    | `in`         | `in`             | `in`     | `in`        | `in`             | `out`                       | `out`     |
|  1 | "Decline"    | -                | -        | -           | -                | "Decline"                   | null      |
|  2 | -            | "Decline"        | -        | -           | -                | "Decline"                   | null      |
|  3 | -            | -                | <1.0     | -           | -                | "Decline"                   | null      |
|  4 | -            | -                | -        | -           | "Decline"        | "Decline"                   | null      |
|  5 | -            | -                | -        | "Elevated"  | -                | "Decline"                   | null      |
|  6 | "A"          | "Strong"         | >=1.50   | "Low"       | "High"           | "T1"                        | 4.25      |
|  7 | "A"          | "Strong"         | >=1.50   | "Low"       | "Standard"       | "T1"                        | 4.50      |
|  8 | "A"          | "Strong"         | >=1.25   | -           | -                | "T2"                        | 5.25      |
|  9 | "A"          | "Adequate"       | >=1.25   | "Low"       | -                | "T2"                        | 5.50      |
| 10 | "A"          | -                | >=1.10   | -           | -                | "T3"                        | 6.50      |
| 11 | "B"          | "Strong"         | >=1.50   | "Low"       | "High"           | "T2"                        | 5.25      |
| 12 | "B"          | "Strong"         | >=1.25   | -           | -                | "T2"                        | 5.75      |
| 13 | "B"          | "Adequate"       | >=1.25   | -           | -                | "T3"                        | 6.50      |
| 14 | "B"          | -                | >=1.10   | -           | -                | "T3"                        | 6.75      |
| 15 | "B"          | "Marginal"       | >=1.00   | -           | -                | "T4"                        | 7.75      |
| 16 | "C"          | "Strong"         | >=1.50   | -           | -                | "T3"                        | 6.50      |
| 17 | "C"          | "Adequate"       | >=1.25   | -           | -                | "T3"                        | 7.00      |
| 18 | "C"          | -                | >=1.10   | -           | -                | "T4"                        | 7.75      |
| 19 | "C"          | "Weak"           | >=1.00   | -           | -                | "T5"                        | 9.00      |
| 20 | "D"          | "Strong"         | >=1.50   | -           | -                | "T4"                        | 7.75      |
| 21 | "D"          | "Adequate"       | >=1.25   | -           | -                | "T4"                        | 8.25      |
| 22 | "D"          | -                | >=1.10   | -           | -                | "T5"                        | 9.00      |
| 23 | "D"          | "Weak"           | >=1.00   | -           | -                | "T5"                        | 9.75      |
| 24 | "E"          | "Strong"         | >=1.50   | "Low"       | -                | "T5"                        | 9.00      |
| 25 | "E"          | "Adequate"       | >=1.25   | "Low"       | -                | "T5"                        | 9.75      |
| 26 | "E"          | -                | -        | -           | -                | "Decline"                   | null      |

Rules 1-5 propagate upstream declines. Rules 6-25 form the pricing
matrix. Rule 26 catches E-grade applicants with insufficient collateral
or DSCR — outside appetite.
