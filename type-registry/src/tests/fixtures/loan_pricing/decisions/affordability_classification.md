---
dmn:
  id: affordability_classification
  type: bkm
  name: Affordability Classification
  data-type:
    ref: AffordabilityClassificationOutput
    schema: ../types/affordability.ts
  signature:
    parameters:
      - name: dtiRatio
        type: number
      - name: stressedDtiRatio
        type: number
      - name: productType
        type: ProductType
      - name: borrowerType
        type: BorrowerType
    return-type: AffordabilityClassificationOutput
    schema: ../types/affordability.ts
  requires:
    - dti_calculation
---

# Table B — Affordability Classification

Classifies DTI and stressed DTI ratios into affordability bands using
**UNIQUE** hit policy. Each combination of inputs must match exactly one
rule — any gap or overlap is a model defect.

## Decision table

| Rule | DTI ratio     | Stressed DTI     | Product type | Borrower type | Band     | Reason |
|------|---------------|------------------|--------------|---------------|----------|--------|
| B1   | ≤ 0.35        | ≤ 0.40           | Any          | Any           | Pass     | AFC-01 |
| B2   | ≤ 0.35        | (0.40, 0.45]     | Any          | Any           | Marginal | AFC-02 |
| B3   | ≤ 0.35        | (0.45, 0.50]     | Any          | Any           | Refer    | AFC-03 |
| B4   | ≤ 0.35        | > 0.50           | Any          | Any           | Fail     | AFC-04 |
| B5   | (0.35, 0.43]  | ≤ 0.43           | Any          | Any           | Marginal | AFC-05 |
| B6   | (0.35, 0.43]  | (0.43, 0.50]     | Any          | Any           | Refer    | AFC-06 |
| B7   | (0.35, 0.43]  | > 0.50           | Any          | Any           | Fail     | AFC-04 |
| B8   | (0.43, 0.50]  | Any              | Mortgage     | Individual    | Refer    | AFC-07 |
| B9   | (0.43, 0.50]  | Any              | Mortgage     | Individual    | Fail     | AFC-08 |
| B10  | (0.43, 0.50]  | Any              | Unsecured    | Any           | Fail     | AFC-04 |
| B11  | (0.43, 0.50]  | Any              | Revolving    | Any           | Fail     | AFC-04 |
| B12  | > 0.50        | Any              | Any          | Any           | Fail     | AFC-09 |
| B13  | Any           | Any              | Buy-to-let   | Any           | —        | —      |

**Known defect:** Rules B8 and B9 have identical input conditions — this is a
deliberate overlap for testing overlap detection. B13 is intentionally incomplete:
buy-to-let goes through a separate ICR path.
