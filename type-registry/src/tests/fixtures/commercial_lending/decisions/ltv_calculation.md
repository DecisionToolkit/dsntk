---
dmn:
  id: ltv_calculation
  type: bkm
  name: LTV Calculation
  signature:
    parameters:
      - name: requestedAmount
        type: number
      - name: valuationAmount
        type: number
    return-type: number
  requires:
    - loan_request
    - property_data
---

# LTV Calculation

Computes Loan-to-Value ratio as a simple division:

```
LTV = requestedAmount / valuationAmount
```

This BKM contains no decision table — the logic is a single FEEL expression.
The result is a decimal between 0 and 1 (e.g. 0.72 = 72% LTV).
