---
dmn:
  id: dti_calculation
  type: bkm
  name: DTI Calculation
  data-type:
    ref: DtiCalculationOutput
    schema: ../types/affordability.ts
  signature:
    parameters:
      - name: grossAnnualIncome
        type: number
      - name: totalMonthlyDebt
        type: number
      - name: requestedMonthlyRepayment
        type: number
      - name: stressTestRate
        type: number
      - name: currentRate
        type: number
      - name: productType
        type: ProductType
    return-type: DtiCalculationOutput
    schema: ../types/affordability.ts
---

# Table A — DTI Calculation

Calculates derived affordability values from raw loan application inputs.

This BKM computes three derived values:

1. **DTI ratio** — `(totalMonthlyDebt + requestedMonthlyRepayment) / (grossAnnualIncome / 12)`
2. **Stressed monthly repayment** — product-type-specific stress calculation
3. **Stressed DTI ratio** — DTI recalculated using the stressed repayment

## Product-type stress rules

| Product type       | Stressed repayment formula                                          |
|--------------------|---------------------------------------------------------------------|
| Mortgage           | `repayment × (1 + (stressRate - currentRate) / currentRate)`        |
| Unsecured term     | `repayment × (stressRate / currentRate)`                            |
| Revolving facility | `limit × 0.03 × (stressRate / currentRate)`                        |
| Buy-to-let         | Uses ICR basis: `rentalIncome × 0.75 / 12` (not DTI)               |

The buy-to-let row uses a completely different calculation basis (interest coverage
ratio, not debt-to-income), which means the downstream classification table needs to
know which basis was used. The `calculationBasis` output field carries this metadata.
