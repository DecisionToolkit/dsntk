---
dmn:
  id: income_verification_modifier
  type: bkm
  name: Income Verification Modifier
  data-type:
    ref: IncomeVerificationOutput
    schema: ../types/affordability.ts
  signature:
    parameters:
      - name: incomeType
        type: IncomeType
      - name: verificationStatus
        type: VerificationStatus
      - name: currentBand
        type: AffordabilityBand
    return-type: IncomeVerificationOutput
    schema: ../types/affordability.ts
  requires:
    - affordability_classification
---

# Table C — Income Verification Modifier

Modifies the affordability band from Table B based on income evidence
quality. Uses **PRIORITY** hit policy — first matching rule wins. Row
order encodes business logic.

## Decision table

| Priority | Income type   | Verification status   | Current band | Adjusted band | Code   |
|----------|---------------|-----------------------|--------------|---------------|--------|
| 1        | Any           | Fraud flag            | Any          | Fail          | INC-99 |
| 2        | Self-employed | Unverified            | Any          | Fail          | INC-01 |
| 3        | Self-employed | 1 year accounts       | Pass         | Marginal      | INC-02 |
| 4        | Self-employed | 1 year accounts       | Marginal     | Refer         | INC-02 |
| 5        | Self-employed | 2+ year accounts      | Any          | No change     | INC-03 |
| 6        | Employed      | Payslip verified      | Any          | No change     | INC-04 |
| 7        | Employed      | Bank statement only   | Pass         | Marginal      | INC-05 |
| 8        | Employed      | Bank statement only   | Marginal     | Refer         | INC-05 |
| 9        | Employed      | Unverified            | Pass         | Refer         | INC-06 |
| 10       | Employed      | Unverified            | Marginal     | Fail          | INC-06 |
| 11       | Contract      | Contract < 6 months   | Any          | Fail          | INC-07 |
| 12       | Contract      | Contract 6–24 months  | Pass         | Marginal      | INC-08 |
| 13       | Contract      | Contract 6–24 months  | Marginal     | Refer         | INC-08 |
| 14       | Contract      | Contract > 24 months  | Any          | No change     | INC-09 |
| 15       | Benefits      | Any                   | Pass         | Refer         | INC-10 |
| 16       | Benefits      | Any                   | Marginal     | Fail          | INC-10 |
| 17       | Any           | Any                   | Any          | No change     | INC-00 |

Rule 17 is the catch-all. A credit officer reading this table must understand
that the order of rows encodes business logic — row 1 always wins, row 17
only fires if nothing above it matched.
