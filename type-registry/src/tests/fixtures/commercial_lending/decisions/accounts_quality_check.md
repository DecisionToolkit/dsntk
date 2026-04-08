---
dmn:
  id: accounts_quality_check
  type: decision
  name: Accounts Quality Check
  requires:
    - financial_statements
  governed-by:
    - commercial_lending_policy
---

# Accounts Quality Check

Validates that the financial statements meet minimum quality thresholds
for the requested facility. Uses **FIRST** hit policy — hard stops are
checked before quality grading.

Stale accounts (older than 18 months) are an automatic decline per policy.
The quality grade feeds into the pricing tier calculation.

## Decision table

> # Accounts Quality Check
> Accounts Quality

| F  | Accounts Type     | Accounts Age | Accounts Quality                    |
|:--:|:-----------------:|:------------:|:-----------------------------------:|
|    |                   |              | "High","Standard","Weak","Decline"  |
|    | `in`              | `in`         | `out`                               |
|  1 | -                 | >18          | "Decline"                           |
|  2 | "Audited"         | <=6          | "High"                              |
|  3 | "Audited"         | (6..12]      | "Standard"                          |
|  4 | "Audited"         | (12..18]     | "Weak"                              |
|  5 | "Management"      | <=6          | "Standard"                          |
|  6 | "Management"      | (6..12]      | "Weak"                              |
|  7 | "Management"      | (12..18]     | "Weak"                              |
|  8 | "Self-Certified"  | <=6          | "Weak"                              |
|  9 | "Self-Certified"  | >6           | "Decline"                           |

Self-certified accounts older than 6 months are declined — the numbers
are too stale to rely on without audit or management sign-off.
