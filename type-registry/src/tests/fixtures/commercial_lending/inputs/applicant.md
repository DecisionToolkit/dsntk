---
dmn:
  id: applicant_data
  type: input-data
  name: Applicant Data
  data-type:
    ref: ApplicantData
    schema: ../types/inputs.ts
---

# Applicant Data

Business applicant details from the commercial lending origination system.

## Expected fields

- **businessName** — registered company name
- **businessType** — "Sole Trader", "Partnership", "Ltd Company", "PLC"
- **yearsTrading** — number of full years in operation
- **sector** — SIC code sector: "Retail", "Manufacturing", "Construction", "Hospitality", "Technology", "Agriculture", "Healthcare", "Professional Services"
- **annualTurnover** — most recent full-year revenue in GBP
- **directorCount** — number of registered directors
- **hasPersonalGuarantee** — whether directors provide personal guarantees
