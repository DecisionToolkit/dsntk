---
dmn:
  id: rate_adjustments
  type: decision
  name: Rate Adjustments
  requires:
    - pricing_tier
    - property_data
    - applicant_data
    - loan_request
  governed-by:
    - commercial_lending_policy
---

# Rate Adjustments

Applies additive basis-point adjustments to the base rate from the
pricing tier. Uses **COLLECT (Sum)** hit policy — all matching rules
fire and their adjustments are summed.

Each rule represents a distinct risk factor that adds to (or subtracts
from) the base rate. The total adjustment is capped at +3.00% per policy.

## Decision table

> # Rate Adjustments
> Rate Adjustment

| C  | Environmental Risk | Flood Zone          | Sector             | Purpose          | Has Guarantee | Term Months | Adjustment | Reason Code |
|:--:|:------------------:|:-------------------:|:------------------:|:----------------:|:------------:|:-----------:|:----------:|:-----------:|
|    | `in`               | `in`                | `in`               | `in`             | `in`         | `in`        | `out`      | `out`       |
|  1 | "Low"              | -                   | -                  | -                | -            | -           | 0.10       | "ENV-01"    |
|  2 | "Medium"           | -                   | -                  | -                | -            | -           | 0.35       | "ENV-02"    |
|  3 | -                  | "Zone 2"            | -                  | -                | -            | -           | 0.15       | "FLD-01"    |
|  4 | -                  | "Zone 3a"           | -                  | -                | -            | -           | 0.50       | "FLD-02"    |
|  5 | -                  | -                   | "Construction"     | -                | -            | -           | 0.50       | "SEC-01"    |
|  6 | -                  | -                   | "Hospitality"      | -                | -            | -           | 0.40       | "SEC-02"    |
|  7 | -                  | -                   | "Retail"           | -                | -            | -           | 0.25       | "SEC-03"    |
|  8 | -                  | -                   | "Agriculture"      | -                | -            | -           | 0.20       | "SEC-04"    |
|  9 | -                  | -                   | -                  | "Development"    | -            | -           | 0.75       | "PUR-01"    |
| 10 | -                  | -                   | -                  | "Working Capital"| -            | -           | 0.15       | "PUR-02"    |
| 11 | -                  | -                   | -                  | -                | true         | -           | -0.25      | "GTR-01"    |
| 12 | -                  | -                   | -                  | -                | -            | >120        | 0.20       | "TRM-01"    |
| 13 | -                  | -                   | -                  | -                | -            | >240        | 0.30       | "TRM-02"    |

Rule 11 is the only negative adjustment — a personal guarantee from
directors reduces the rate. Rules 12 and 13 can stack: a 25-year term
(300 months) triggers both TRM-01 (+0.20%) and TRM-02 (+0.30%) for a
combined +0.50% term loading.
