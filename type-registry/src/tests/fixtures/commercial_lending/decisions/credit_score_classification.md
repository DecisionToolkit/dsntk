---
dmn:
  id: credit_score_classification
  type: decision
  name: Credit Score Classification
  requires:
    - bureau_report
    - applicant_data
  governed-by:
    - commercial_lending_policy
---

# Credit Score Classification

Classifies the commercial bureau score into a risk grade, adjusted for
business maturity and filing compliance. Uses **FIRST** hit policy — row
order encodes precedence, with hard-stop declines checked before any
grading.

## Decision table

> # Credit Score Classification
> Credit Grade

| F  | Bureau Score | CCJ Count | CCJ Total Value | Filing Status | Years Trading | Credit Grade                          |
|:--:|:------------:|:---------:|:---------------:|:-------------:|:-------------:|:-------------------------------------:|
|    |              |           |                 |               |               | "A","B","C","D","E",*"Decline"*       |
|    | `in`         | `in`      | `in`            | `in`          | `in`          | `out`                                 |
|  1 | <30          | -         | -               | -             | -             | "Decline"                             |
|  2 | -            | -         | >50000          | -             | -             | "Decline"                             |
|  3 | -            | >3        | -               | -             | -             | "Decline"                             |
|  4 | -            | -         | -               | "Dormant"     | -             | "Decline"                             |
|  5 | >=80         | 0         | 0               | "Current"     | >=5           | "A"                                   |
|  6 | >=80         | 0         | 0               | "Current"     | [3..5)        | "B"                                   |
|  7 | [65..80)     | 0         | 0               | "Current"     | >=3           | "B"                                   |
|  8 | [65..80)     | <=1       | <=5000          | "Current"     | >=5           | "C"                                   |
|  9 | [50..65)     | <=1       | <=10000         | "Current"     | >=3           | "C"                                   |
| 10 | [50..65)     | <=2       | <=20000         | -             | >=2           | "D"                                   |
| 11 | [40..50)     | <=2       | <=30000         | -             | >=2           | "D"                                   |
| 12 | [30..40)     | -         | <=50000         | -             | >=1           | "E"                                   |
| 13 | -            | -         | -               | -             | <1            | "E"                                   |
| 14 | -            | -         | -               | -             | -             | "D"                                   |

Rules 1-4 are hard stops that produce an automatic decline regardless of
other factors. Rule 14 is the catch-all for any application that passes
hard stops but doesn't match a higher grade.
