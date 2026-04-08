# Commercial Lending Decision Model

A complex multi-level DMN decision model for commercial loan origination.
Each file contains one decision table with YAML front matter declaring
its DMN node type, dependencies, and governance.

## Decision Requirements Graph

```
                        ┌─────────────────┐
                        │  Loan Request    │ (input-data)
                        └────────┬────────┘
                                 │
    ┌──────────────┐    ┌────────┴────────┐    ┌────────────────────┐    ┌────────────────┐
    │ Applicant    │    │ Property Data   │    │ Financial          │    │ Bureau Report  │
    │ Data         │    │                 │    │ Statements         │    │                │
    └──┬───┬───┬───┘    └──┬────┬────┬────┘    └──┬────┬────┬───┬──┘    └──┬─────────────┘
       │   │   │           │    │    │            │    │    │   │           │
       │   │   │           │    │    │            │    │    │   │           │
  ┌────┘   │   │      ┌───┘    │    └───┐    ┌──┘    │    │   └──┐    ┌──┘
  │        │   │      │        │        │    │       │    │      │    │
  v        │   │      v        │        v    v       │    v      │    v
┌──────────┴───┴──┐ ┌─────────┴──────┐ ┌────┴───────┴──┐ ┌──────┴────────────┐
│ Sector Risk     │ │ Property Risk  │ │ DSCR          │ │ Credit Score      │
│ Assessment      │ │ Rating         │ │ Calculation   │ │ Classification    │
│ (U, 22 rules)   │ │ (U, 19 rules) │ │ (BKM)         │ │ (F, 14 rules)     │
└────────┬────────┘ └────┬───────────┘ └────┬──────────┘ └────┬──────────────┘
         │               │                  │                  │
         │          ┌────┴─────────┐        │                  │
         │          v              v        │                  │
         │    ┌──────────────────────┐      │                  │
         │    │ Collateral Adequacy  │◄─────┘                  │
         │    │ (U, 14 rules)        │                         │
         │    └──────────┬───────────┘                         │
         │               │                                     │
    ┌────┘               │            ┌────────────────────┐   │
    │    ┌───────────────┘            │ Accounts Quality   │   │
    │    │                            │ Check (F, 9 rules) │   │
    │    │                            └────────┬───────────┘   │
    v    v                                     v               v
  ┌──────────────────────────────────────────────────────────────┐
  │                      Pricing Tier                            │
  │                   (P, 26 rules)                              │
  └──────────────────────────┬───────────────────────────────────┘
                             │
                    ┌────────┴────────┐
                    v                 v
          ┌─────────────────┐  ┌──────────────────────────────────┐
          │ Rate Adjustments│  │         Final Approval            │
          │ (C+, 13 rules)  │  │         (F, 17 rules)             │
          └────────┬────────┘  └──────────────┬────────────────────┘
                   │                          │
                   └──────────┬───────────────┘
                              v
                    ┌──────────────────────────┐
                    │  Conditions of Approval   │
                    │  (C, 16 rules)            │
                    └───────────────────────────┘
```

## Files

### Inputs (5 files)
| File | Node | Description |
|------|------|-------------|
| `inputs/applicant.md` | Applicant Data | Business details and director info |
| `inputs/property.md` | Property Data | Collateral property characteristics |
| `inputs/financial_statements.md` | Financial Statements | Audited/management accounts |
| `inputs/bureau_report.md` | Bureau Report | Commercial credit bureau data |
| `inputs/loan_request.md` | Loan Request | Facility amount, term, purpose |

### Decisions (8 files)
| File | Hit Policy | Rules | Dependencies |
|------|-----------|-------|-------------|
| `decisions/credit_score_classification.md` | First | 14 | Bureau Report, Applicant |
| `decisions/property_risk_rating.md` | Unique | 19 | Property |
| `decisions/sector_risk_assessment.md` | Unique | 22 | Applicant, Financials |
| `decisions/accounts_quality_check.md` | First | 9 | Financials |
| `decisions/collateral_adequacy.md` | Unique | 14 | LTV, Property Risk, Loan Request |
| `decisions/pricing_tier.md` | Priority | 26 | Credit, Collateral, DSCR, Sector, Accounts |
| `decisions/rate_adjustments.md` | Collect+ | 13 | Pricing Tier, Property, Applicant, Loan |
| `decisions/final_approval.md` | First | 17 | Pricing Tier, all upstream |
| `decisions/conditions_of_approval.md` | Collect | 16 | Final Approval, all upstream |

### BKMs (2 files)
| File | Description |
|------|-------------|
| `decisions/ltv_calculation.md` | Loan-to-Value ratio (expression, no table) |
| `decisions/dscr_calculation.md` | Debt Service Coverage Ratio (expression, no table) |

### Knowledge Sources (1 file)
| File | Description |
|------|-------------|
| `knowledge_sources/commercial_lending_policy.md` | Bank's commercial credit policy |

## Hit policies used

- **F (First)** — Credit Score Classification, Accounts Quality, Final Approval
- **U (Unique)** — Property Risk Rating, Sector Risk Assessment, Collateral Adequacy
- **P (Priority)** — Pricing Tier
- **C+ (Collect Sum)** — Rate Adjustments
- **C (Collect)** — Conditions of Approval

## Total: 150 rules across 9 decision tables, 4 levels of dependency depth
