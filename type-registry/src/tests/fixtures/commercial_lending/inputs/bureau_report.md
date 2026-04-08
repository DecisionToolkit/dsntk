---
dmn:
  id: bureau_report
  type: input-data
  name: Bureau Report
  data-type:
    ref: BureauReport
    schema: ../types/inputs.ts
---

# Bureau Report

Commercial credit bureau data (Dun & Bradstreet / Experian Business).

## Expected fields

- **bureauScore** — commercial credit score (0-100)
- **ccjCount** — number of County Court Judgments in last 6 years
- **ccjTotalValue** — total value of CCJs in GBP
- **paymentDaysAverage** — average days to pay suppliers (DBT)
- **filingStatus** — "Current", "Overdue", "Dormant"
- **directorDefaults** — number of director-linked personal defaults
