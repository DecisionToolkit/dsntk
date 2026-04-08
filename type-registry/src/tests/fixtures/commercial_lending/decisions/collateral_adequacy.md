---
dmn:
  id: collateral_adequacy
  type: decision
  name: Collateral Adequacy
  requires:
    - ltv_calculation
    - property_risk_rating
    - loan_request
  governed-by:
    - commercial_lending_policy
---

# Collateral Adequacy

Combines the LTV ratio with the property risk rating to determine whether
the collateral is sufficient security for the facility. Uses **UNIQUE**
hit policy.

The LTV thresholds vary by property risk — a low-risk prime office can
support higher leverage than a high-risk industrial unit.

## Decision table

> # Collateral Adequacy
> Collateral Grade

| U  | LTV          | Property Risk    | Purpose            | Collateral Grade                                 |
|:--:|:------------:|:----------------:|:------------------:|:------------------------------------------------:|
|    |              |                  |                    | "Strong","Adequate","Marginal","Weak","Decline"  |
|    | `in`         | `in`             | `in`               | `out`                                            |
|  1 | -            | "Unacceptable"   | -                  | "Decline"                                        |
|  2 | >0.85        | -                | -                  | "Decline"                                        |
|  3 | >0.65        | -                | "Development"      | "Decline"                                        |
|  4 | <=0.50       | "Low"            | -                  | "Strong"                                         |
|  5 | (0.50..0.65] | "Low"            | -                  | "Adequate"                                       |
|  6 | (0.65..0.75] | "Low"            | -                  | "Marginal"                                       |
|  7 | (0.75..0.85] | "Low"            | -                  | "Weak"                                           |
|  8 | <=0.50       | "Medium"         | -                  | "Adequate"                                       |
|  9 | (0.50..0.65] | "Medium"         | -                  | "Marginal"                                       |
| 10 | (0.65..0.75] | "Medium"         | -                  | "Weak"                                           |
| 11 | (0.75..0.85] | "Medium"         | -                  | "Decline"                                        |
| 12 | <=0.50       | "High"           | -                  | "Marginal"                                       |
| 13 | (0.50..0.65] | "High"           | -                  | "Weak"                                           |
| 14 | >0.65        | "High"           | -                  | "Decline"                                        |

Rules 1-3 are hard stops inherited from the policy: unacceptable property,
LTV above 85%, or development above 65% LTV are automatic declines.
