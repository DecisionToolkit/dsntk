---
dmn:
  id: loan_application_input
  type: input-data
  name: Loan Application
  data-type:
    ref: LoanApplication
    schema: ../types/applicant.ts
---

# Loan Application Input

Raw application data from the loan origination system.

This input-data node represents the boundary where the decision engine
meets the outside world. Fields are populated by the core banking system
at application submission time.
