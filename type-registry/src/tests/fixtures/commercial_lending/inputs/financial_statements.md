---
dmn:
  id: financial_statements
  type: input-data
  name: Financial Statements
  data-type:
    ref: FinancialStatements
    schema: ../types/inputs.ts
---

# Financial Statements

Audited or management accounts for the applicant business.

## Expected fields

- **netProfit** — net profit after tax for the most recent year in GBP
- **depreciation** — annual depreciation charge in GBP
- **interestExpense** — annual interest payments on existing debt in GBP
- **totalDebt** — total outstanding borrowings in GBP
- **totalAssets** — total asset value in GBP
- **accountsType** — "Audited", "Management", "Self-Certified"
- **accountsAge** — months since accounts date
