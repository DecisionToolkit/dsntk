---
dmn:
  id: affordability_result
  type: decision
  name: Affordability Result
  data-type:
    ref: AffordabilityResult
    schema: ../types/affordability.ts
  requires:
    - dti_calculation
    - affordability_classification
    - income_verification_modifier
  governed-by:
    - credit_policy_framework
---

# Affordability Result

Assembles the outputs of the three affordability sub-tables (DTI Calculation,
Affordability Classification, Income Verification Modifier) into a single
composite result with a complete audit trail.

## Audit trail requirements

The `auditTrail` field must carry sufficient detail to answer the question
"why was this application declined?" per FCA Consumer Duty / ECOA requirements:

- `tableAOutput` — which stress calculation basis was used (DTI or ICR)
- `tableBBand` — the raw affordability band before income modification
- `tableBReason` — the reason code from Table B (e.g. AFC-07)
- `tableCModifier` — the income modifier code from Table C (e.g. INC-02)
- `tableCOriginalBand` — the band before Table C modified it

The final reason code composition must be machine-readable and human-legible:
not just "declined due to risk rating" but "declined because affordability
sub-score failed stress test at 7.5% with DTI of 48%, producing risk band D,
below minimum threshold for this product type in this jurisdiction."
