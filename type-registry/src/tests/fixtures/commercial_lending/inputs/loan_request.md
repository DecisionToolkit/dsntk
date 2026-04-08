---
dmn:
  id: loan_request
  type: input-data
  name: Loan Request
  data-type:
    ref: LoanRequest
    schema: ../types/inputs.ts
---

# Loan Request

The specific loan facility being requested.

## Expected fields

- **requestedAmount** — facility amount in GBP
- **termMonths** — requested term in months
- **purpose** — "Purchase", "Refinance", "Development", "Working Capital", "Equipment"
- **repaymentType** — "Capital and Interest", "Interest Only", "Bullet"
