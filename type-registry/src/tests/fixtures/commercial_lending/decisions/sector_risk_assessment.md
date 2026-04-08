---
dmn:
  id: sector_risk_assessment
  type: decision
  name: Sector Risk Assessment
  requires:
    - applicant_data
    - financial_statements
  governed-by:
    - commercial_lending_policy
---

# Sector Risk Assessment

Evaluates the risk associated with the applicant's business sector,
adjusted for business maturity and financial strength. Uses **UNIQUE**
hit policy.

Certain sectors carry inherently higher risk due to cyclicality,
regulatory exposure, or market conditions. The accounts quality
provides a secondary signal — self-certified accounts in a high-risk
sector compound the uncertainty.

## Decision table

> # Sector Risk Assessment
> Sector Risk

| U  | Sector                 | Years Trading | Accounts Type    | Total Debt / Total Assets | Sector Risk                       |
|:--:|:----------------------:|:-------------:|:----------------:|:-------------------------:|:---------------------------------:|
|    |                        |               |                  |                           | "Low","Medium","High","Elevated"  |
|    | `in`                   | `in`          | `in`             | `in`                      | `out`                             |
|  1 | "Technology"           | >=5           | "Audited"        | <=0.60                    | "Low"                             |
|  2 | "Technology"           | >=3           | "Audited"        | <=0.70                    | "Medium"                          |
|  3 | "Technology"           | <3            | -                | -                         | "High"                            |
|  4 | "Healthcare"           | >=3           | -                | <=0.70                    | "Low"                             |
|  5 | "Healthcare"           | <3            | -                | -                         | "Medium"                          |
|  6 | "Professional Services"| >=5           | "Audited"        | <=0.50                    | "Low"                             |
|  7 | "Professional Services"| >=2           | -                | <=0.70                    | "Medium"                          |
|  8 | "Professional Services"| <2            | -                | -                         | "High"                            |
|  9 | "Manufacturing"        | >=5           | "Audited"        | <=0.60                    | "Medium"                          |
| 10 | "Manufacturing"        | >=3           | -                | <=0.75                    | "High"                            |
| 11 | "Manufacturing"        | <3            | -                | -                         | "Elevated"                        |
| 12 | "Retail"               | >=5           | "Audited"        | <=0.50                    | "Medium"                          |
| 13 | "Retail"               | >=3           | -                | <=0.65                    | "High"                            |
| 14 | "Retail"               | <3            | -                | -                         | "Elevated"                        |
| 15 | "Construction"         | >=10          | "Audited"        | <=0.50                    | "High"                            |
| 16 | "Construction"         | -             | -                | -                         | "Elevated"                        |
| 17 | "Hospitality"          | >=5           | "Audited"        | <=0.50                    | "High"                            |
| 18 | "Hospitality"          | -             | -                | -                         | "Elevated"                        |
| 19 | "Agriculture"          | >=5           | -                | <=0.60                    | "Medium"                          |
| 20 | "Agriculture"          | <5            | -                | -                         | "High"                            |
| 21 | -                      | -             | "Self-Certified"  | -                         | "Elevated"                        |
| 22 | -                      | -             | -                | >0.80                     | "Elevated"                        |

Rule 21 catches any sector where the accounts are self-certified — this
always elevates risk regardless of sector strength. Rule 22 catches
extreme leverage in any sector.
