# Evaluation Gaps: FEEL Expression BKMs, Data Fixes, and Downstream Resolution

**Goal:** Complete the evaluation pipeline so all 9 decision/BKM nodes in the commercial lending project produce correct results when evaluated in topological order.

**Context:** The project viewer can load markdown-based DMN projects, render the DRG, apply scenario presets, and evaluate decision tables with rule highlighting. Three gaps remain: expression-only BKMs produce no results, a missing input field, and one hit policy overlap.

---

## Gap 1: FEEL Expression BKMs

### Problem

`ltv_calculation` and `dscr_calculation` are BKMs that contain a FEEL expression rather than a decision table. The evaluate handler currently attempts `from_markdown()` on all BKMs, which fails silently for expression-only nodes. Downstream decisions (Collateral Adequacy, Pricing Tier) never receive LTV or DSCR values.

### Solution

Add an optional `feel-expression` field to the YAML front matter. The evaluate handler checks for this field before attempting table recognition.

**Front matter example (LTV):**

```yaml
dmn:
  id: ltv_calculation
  type: bkm
  name: LTV Calculation
  feel-expression: Requested Amount / Valuation Amount
  requires:
    - loan_request
    - property_data
```

**Front matter example (DSCR):**

```yaml
dmn:
  id: dscr_calculation
  type: bkm
  name: DSCR Calculation
  feel-expression: (Net Profit + Depreciation + Interest Expense) / Annual Debt Service
  requires:
    - financial_statements
    - loan_request
```

**Evaluate handler logic:**

1. Read the `feel-expression` from the node's `DmnMetadata`.
2. If present: parse the expression via `dsntk_feel_parser::parse_expression()`, prepare an evaluator via `dsntk_feel_evaluator::prepare()`, evaluate against the current scope.
3. Store the result in scope under the BKM name (e.g., "LTV Calculation") and under the `output-name` field from front matter:
   - `output-name: LTV` for ltv_calculation
   - `output-name: DSCR` for dscr_calculation
   These match the column headers in downstream decision tables (Collateral Adequacy uses "LTV", Pricing Tier uses "DSCR").
4. Return a `NodeEvalResult` with `matched_rules: []` and `output_value: <computed value>`.

**No parameter mapping needed.** The expression uses FEEL names directly from scope. The TypeRegistry already resolved all input fields to their FEEL names (Title Case with spaces), and those are flattened into the scope at evaluation start. The expression references them directly (e.g., `Requested Amount`, `Valuation Amount`).

**The `signature` block becomes optional** for expression BKMs. It can remain for documentation purposes but the handler does not use it for parameter binding.

### Files affected

| File | Change |
|------|--------|
| `type-registry/src/front_matter.rs` (or scanner) | Add `feel_expression: Option<String>` and `output_name: Option<String>` to `DmnMetadata` |
| `server/src/project_handlers.rs` | Add expression evaluation branch before table recognition |
| `type-registry/src/tests/fixtures/commercial_lending/decisions/ltv_calculation.md` | Add `feel-expression` and `output-name` to front matter |
| `type-registry/src/tests/fixtures/commercial_lending/decisions/dscr_calculation.md` | Add `feel-expression` and `output-name` to front matter |

---

## Gap 2: Missing "Annual Debt Service" Input Field

### Problem

The DSCR expression requires `Annual Debt Service`, but this field does not exist in the TypeRegistry types or scenario data. It represents the annual repayment obligation on the proposed facility.

### Solution

Add `Annual Debt Service` as an explicit field in the `FinancialStatements` TypeScript interface and in all 5 scenarios.

**Type addition:**

```typescript
export interface FinancialStatements {
  // ... existing fields ...
  "Annual Debt Service": number;
}
```

**Scenario values** (approximate from Requested Amount / Term Months * 12):

| Scenario | Requested Amount | Term Months | Annual Debt Service |
|----------|-----------------|-------------|-------------------|
| Strong Applicant | 350,000 | 60 | 70,000 |
| Marginal | 750,000 | 180 | 50,000 |
| Decline | 500,000 | 120 | 50,000 |
| Edge Case | 5,000,000 | 300 | 200,000 |
| Sole Trader | 25,000 | 36 | 8,333 |

### Files affected

| File | Change |
|------|--------|
| `type-registry/src/tests/fixtures/commercial_lending/types/inputs.ts` | Add `"Annual Debt Service": number` |
| `type-registry/src/tests/fixtures/commercial_lending/scenarios.json` | Add `"Annual Debt Service"` to all 5 scenarios |

---

## Gap 3: Collateral Adequacy Hit Policy Overlap

### Problem

Collateral Adequacy uses UNIQUE (`U`) hit policy. For some inputs, multiple rules match (e.g., LTV <=0.50 with "Low" Property Risk matches rule 4, while the catch-all decline rules may also overlap). This causes `err_multiple_rules_match_in_unique_hit_policy` and a null result.

### Solution

Change the hit policy from `U` (UNIQUE) to `F` (FIRST). The rules are already ordered by priority: decline rules first (1-3), then strong → weak grades (4-14). FIRST hit policy respects this ordering.

### Files affected

| File | Change |
|------|--------|
| `type-registry/src/tests/fixtures/commercial_lending/decisions/collateral_adequacy.md` | Change `U` to `F` in table header |

---

## Gap 4: Frontend — Expression BKM Node Display

### Problem

Expression BKM nodes currently render as table nodes with empty rule rows. They should display the expression and computed value instead.

### Solution

Extend the `ProjectNode::Bkm` variant (or the project endpoint response) to include an optional `feel_expression: Option<String>` field. The `ProjectBkmNode.svelte` component checks: if `feel_expression` is present, render the expression text; otherwise render the table rows as today.

When evaluated, the node shows the computed value in the eval footer (same green highlight pattern as table nodes).

### Files affected

| File | Change |
|------|--------|
| `server/src/project_handlers.rs` | Include `feel_expression` in BKM node response |
| `viewer/src/lib/types.ts` | Add optional `feel_expression` to BKM node type |
| `viewer/src/lib/components/ProjectBkmNode.svelte` | Render expression text when present |

---

## Testing

### Unit tests (model-evaluator)

Add to `model-evaluator/src/tests/markdown_eval.rs`:

- Test FEEL expression parsing and evaluation with multi-word names in scope
- Test that expression result is a number (LTV, DSCR patterns)
- Test expression with division-by-zero returns null

### Integration test (server)

- Evaluate the "Strong Applicant" scenario end-to-end, verify:
  - LTV = 350000 / 750000 = 0.4667
  - DSCR = (680000 + 45000 + 32000) / 70000 = 10.81
  - Collateral Adequacy matches (LTV ~0.47, Property Risk "Low") → "Strong"
  - Pricing Tier matches (Credit Grade "A", Collateral Grade "Strong", DSCR 10.81, SectorRisk "Low", Accounts Quality "High") → "T1"
  - Final Approval → "Approved" (Auto)

---

## Summary of all changes

| # | Change | Files |
|---|--------|-------|
| 1 | Add `feel-expression` and `output-name` to front matter parser | `front_matter.rs` |
| 2 | Add expression evaluation branch in evaluate handler | `project_handlers.rs` |
| 3 | Update LTV and DSCR markdown files with front matter fields | `ltv_calculation.md`, `dscr_calculation.md` |
| 4 | Add "Annual Debt Service" to types and scenarios | `inputs.ts`, `scenarios.json` |
| 5 | Change Collateral Adequacy hit policy U → F | `collateral_adequacy.md` |
| 6 | Add expression display to BKM node frontend | `project_handlers.rs`, `types.ts`, `ProjectBkmNode.svelte` |
| 7 | Add regression tests | `markdown_eval.rs` |
