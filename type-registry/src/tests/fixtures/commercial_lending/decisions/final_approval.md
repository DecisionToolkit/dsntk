---
dmn:
  id: final_approval
  type: decision
  name: Final Approval
  requires:
    - pricing_tier
    - rate_adjustments
    - collateral_adequacy
    - credit_score_classification
    - dscr_calculation
    - sector_risk_assessment
    - accounts_quality_check
    - loan_request
    - applicant_data
  governed-by:
    - commercial_lending_policy
---

# Final Approval

Determines the lending decision by combining all upstream assessments.
Uses **FIRST** hit policy — declines are checked first, then referrals,
then approvals by tier.

This is the penultimate decision in the graph. Its output feeds into
the Conditions of Approval table, which attaches covenants and
monitoring requirements to approved or referred facilities.

## Decision table

> # Final Approval
> Approval Decision

| F  | Pricing Tier | Credit Grade | Collateral Grade | DSCR        | Sector Risk | Requested Amount | Business Type  | Decision                          | Authority Level      |
|:--:|:------------:|:------------:|:----------------:|:-----------:|:-----------:|:----------------:|:--------------:|:---------------------------------:|:--------------------:|
|    |              |              |                  |             |             |                  |                | "Approved","Referred","Declined"  |                      |
|    | `in`         | `in`         | `in`             | `in`        | `in`        | `in`             | `in`           | `out`                             | `out`                |
|  1 | "Decline"    | -            | -                | -           | -           | -                | -              | "Declined"                        | "System"             |
|  2 | -            | "Decline"    | -                | -           | -           | -                | -              | "Declined"                        | "System"             |
|  3 | -            | -            | "Decline"        | -           | -           | -                | -              | "Declined"                        | "System"             |
|  4 | -            | -            | -                | <1.0        | -           | -                | -              | "Declined"                        | "System"             |
|  5 | "T1","T2"    | "A","B"      | "Strong"         | >=1.25      | "Low"       | <=500000         | -              | "Approved"                        | "Auto"               |
|  6 | "T1","T2"    | "A","B"      | -                | >=1.25      | "Low"       | <=250000         | -              | "Approved"                        | "Auto"               |
|  7 | "T1","T2"    | "A","B"      | -                | >=1.25      | -           | <=500000         | -              | "Approved"                        | "Credit Manager"     |
|  8 | "T1","T2"    | -            | -                | >=1.10      | -           | <=1000000        | -              | "Approved"                        | "Senior Manager"     |
|  9 | "T3"         | -            | "Strong","Adequate" | >=1.25   | -           | <=500000         | -              | "Approved"                        | "Credit Manager"     |
| 10 | "T3"         | -            | -                | >=1.10      | -           | <=1000000        | -              | "Approved"                        | "Senior Manager"     |
| 11 | "T3"         | -            | -                | -           | -           | >1000000         | -              | "Referred"                        | "Credit Committee"   |
| 12 | "T4"         | -            | "Strong","Adequate" | >=1.25   | -           | <=500000         | -              | "Approved"                        | "Senior Manager"     |
| 13 | "T4"         | -            | -                | >=1.10      | -           | -                | -              | "Referred"                        | "Credit Committee"   |
| 14 | "T5"         | -            | "Strong"         | >=1.50      | -           | <=250000         | -              | "Referred"                        | "Credit Committee"   |
| 15 | "T5"         | -            | -                | -           | -           | -                | -              | "Referred"                        | "Board"              |
| 16 | -            | -            | -                | -           | -           | >2000000         | "Sole Trader"  | "Referred"                        | "Board"              |
| 17 | -            | -            | -                | -           | -           | -                | -              | "Referred"                        | "Credit Committee"   |

Rules 1-4 are automatic system declines. Rules 5-6 are auto-approvals
for the best-quality applications. Rule 16 ensures sole traders
requesting over 2M always go to the board. Rule 17 is the catch-all —
anything that doesn't match a specific rule gets referred to the
credit committee for manual review.
