---
dmn:
  id: property_data
  type: input-data
  name: Property Data
  data-type:
    ref: PropertyData
    schema: ../types/inputs.ts
---

# Property Data

Commercial property details for the collateral assessment.

## Expected fields

- **propertyType** — "Office", "Retail Unit", "Industrial", "Mixed Use", "Land", "Development Site"
- **valuationAmount** — surveyor's assessed market value in GBP
- **location** — "Prime", "Secondary", "Tertiary", "Rural"
- **tenure** — "Freehold", "Leasehold"
- **leaseRemaining** — years remaining on lease (null if freehold)
- **environmentalRisk** — "None", "Low", "Medium", "High"
- **floodZone** — "Zone 1", "Zone 2", "Zone 3a", "Zone 3b"
