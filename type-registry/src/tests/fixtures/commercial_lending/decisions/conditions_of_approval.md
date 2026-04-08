---
dmn:
  id: conditions_of_approval
  type: decision
  name: Conditions of Approval
  requires:
    - final_approval
    - pricing_tier
    - collateral_adequacy
    - dscr_calculation
    - sector_risk_assessment
    - accounts_quality_check
    - property_data
    - loan_request
  governed-by:
    - commercial_lending_policy
---

# Conditions of Approval

Attaches covenants, monitoring requirements, and special conditions to
approved or referred facilities. Uses **COLLECT** hit policy — all
matching rules fire and their conditions accumulate into a list.

A declined application produces no conditions. Every approved or
referred facility gets at least one condition (the annual review in
rule 16 is a universal requirement).

## Decision table

> # Conditions of Approval
> Conditions

| C  | Decision              | Pricing Tier | Collateral Grade | DSCR        | Sector Risk | Accounts Quality | Property Risk    | Term Months | Condition                                          | Category     |
|:--:|:---------------------:|:------------:|:----------------:|:-----------:|:-----------:|:----------------:|:----------------:|:-----------:|:--------------------------------------------------:|:------------:|
|    | `in`                  | `in`         | `in`             | `in`        | `in`        | `in`             | `in`             | `in`        | `out`                                              | `out`        |
|  1 | "Declined"            | -            | -                | -           | -           | -                | -                | -           | -                                                  | -            |
|  2 | "Approved","Referred" | -            | "Marginal","Weak"| -           | -           | -                | -                | -           | "Independent valuation within 12 months"           | "Collateral" |
|  3 | "Approved","Referred" | -            | "Weak"           | -           | -           | -                | -                | -           | "Additional security or personal guarantee required"| "Collateral"|
|  4 | "Approved","Referred" | -            | -                | [1.00..1.10)| -           | -                | -                | -           | "Quarterly DSCR monitoring covenant"               | "Financial"  |
|  5 | "Approved","Referred" | -            | -                | [1.10..1.25)| -           | -                | -                | -           | "Semi-annual DSCR monitoring covenant"             | "Financial"  |
|  6 | "Approved","Referred" | -            | -                | -           | "High"      | -                | -                | -           | "Enhanced sector monitoring — quarterly reporting"  | "Monitoring" |
|  7 | "Approved","Referred" | -            | -                | -           | -           | "Weak"           | -                | -           | "Updated accounts required within 90 days"         | "Financial"  |
|  8 | "Approved","Referred" | -            | -                | -           | -           | -                | "High"           | -           | "Annual property inspection"                       | "Collateral" |
|  9 | "Approved","Referred" | -            | -                | -           | -           | -                | "Medium"         | >120        | "Property revaluation at year 5"                   | "Collateral" |
| 10 | "Approved","Referred" | "T4","T5"    | -                | -           | -           | -                | -                | -           | "Minimum DSCR covenant: 1.15x"                     | "Financial"  |
| 11 | "Approved","Referred" | "T5"         | -                | -           | -           | -                | -                | -           | "Personal guarantee from all directors"            | "Security"   |
| 12 | "Approved","Referred" | -            | -                | -           | -           | -                | -                | >180        | "Maximum LTV covenant: 75% at year 10"             | "Collateral" |
| 13 | "Referred"            | -            | -                | -           | -           | -                | -                | -           | "Manual underwriter review required"               | "Process"    |
| 14 | "Approved","Referred" | "T3","T4","T5"| -               | -           | -           | -                | -                | -           | "Arrangement fee: 1.5% of facility"                | "Fee"        |
| 15 | "Approved","Referred" | "T1","T2"    | -                | -           | -           | -                | -                | -           | "Arrangement fee: 0.75% of facility"               | "Fee"        |
| 16 | "Approved","Referred" | -            | -                | -           | -           | -                | -                | -           | "Annual facility review"                           | "Monitoring" |

Rule 1 is a no-op for declined applications (COLLECT ignores null
outputs). Rule 16 fires for every approved or referred facility — it's
the universal annual review condition. A typical T4 facility with
marginal collateral and borderline DSCR might accumulate 6-8 conditions.
