---
dmn:
  id: property_risk_rating
  type: decision
  name: Property Risk Rating
  requires:
    - property_data
  governed-by:
    - commercial_lending_policy
---

# Property Risk Rating

Assigns a risk rating to the collateral property based on its physical
characteristics and location. Uses **UNIQUE** hit policy — every property
must match exactly one rule.

## Decision table

> # Property Risk Rating
> Property Risk

| U  | Property Type      | Location      | Tenure      | Lease Remaining | Flood Zone          | Environmental Risk | Property Risk                        |
|:--:|:------------------:|:------------:|:-----------:|:---------------:|:-------------------:|:------------------:|:------------------------------------:|
|    |                    |              |             |                 |                     |                    | "Low","Medium","High","Unacceptable" |
|    | `in`               | `in`         | `in`        | `in`            | `in`                | `in`               | `out`                                |
|  1 | "Office"           | "Prime"      | "Freehold"  | -               | "Zone 1"            | "None"             | "Low"                                |
|  2 | "Office"           | "Prime"      | "Leasehold" | >=25            | "Zone 1"            | "None","Low"       | "Low"                                |
|  3 | "Office"           | "Prime"      | "Leasehold" | [15..25)        | "Zone 1","Zone 2"   | "None","Low"       | "Medium"                             |
|  4 | "Office"           | "Secondary"  | -           | -               | "Zone 1","Zone 2"   | "None","Low"       | "Medium"                             |
|  5 | "Retail Unit"      | "Prime"      | -           | >=20            | "Zone 1"            | "None","Low"       | "Low"                                |
|  6 | "Retail Unit"      | "Prime"      | -           | [10..20)        | "Zone 1","Zone 2"   | "None","Low"       | "Medium"                             |
|  7 | "Retail Unit"      | "Secondary"  | -           | >=15            | "Zone 1","Zone 2"   | "None","Low"       | "Medium"                             |
|  8 | "Retail Unit"      | "Tertiary"   | -           | -               | -                   | -                  | "High"                               |
|  9 | "Industrial"       | -            | "Freehold"  | -               | "Zone 1","Zone 2"   | "None","Low"       | "Medium"                             |
| 10 | "Industrial"       | -            | "Leasehold" | >=20            | "Zone 1","Zone 2"   | "None","Low"       | "Medium"                             |
| 11 | "Industrial"       | -            | -           | -               | -                   | "Medium"           | "High"                               |
| 12 | "Mixed Use"        | "Prime"      | -           | >=15            | "Zone 1","Zone 2"   | "None","Low"       | "Medium"                             |
| 13 | "Mixed Use"        | -            | -           | -               | -                   | -                  | "High"                               |
| 14 | "Land"             | -            | -           | -               | "Zone 1"            | "None"             | "High"                               |
| 15 | "Development Site" | -            | -           | -               | -                   | -                  | "Unacceptable"                       |
| 16 | -                  | "Rural"      | -           | -               | -                   | -                  | "High"                               |
| 17 | -                  | -            | -           | -               | "Zone 3a","Zone 3b" | -                  | "Unacceptable"                       |
| 18 | -                  | -            | -           | -               | -                   | "High"             | "Unacceptable"                       |
| 19 | -                  | -            | "Leasehold" | <10             | -                   | -                  | "High"                               |

Rules 15, 17, and 18 are hard blocks — development sites without planning,
severe flood zones, and high environmental contamination are outside
appetite regardless of other factors.
