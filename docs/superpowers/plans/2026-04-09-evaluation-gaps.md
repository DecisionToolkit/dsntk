# Evaluation Gaps Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the evaluation pipeline so all 9 decision/BKM nodes produce correct results, including expression-only BKMs (LTV, DSCR).

**Architecture:** Add `feel-expression` and `output-name` fields to YAML front matter. The evaluate handler checks for `feel-expression` before attempting table recognition — if present, it parses and evaluates the FEEL expression against the current scope, storing the result under `output-name`. Data gaps (Annual Debt Service, hit policy overlaps) are fixed in the markdown fixtures.

**Tech Stack:** Rust (serde/YAML front matter, dsntk_feel_parser, dsntk_feel_evaluator), Svelte 5 (BKM node display)

---

## File Structure

| File | Responsibility |
|------|---------------|
| `type-registry/src/front_matter.rs` | YAML front matter parsing — add `feel_expression` and `output_name` fields |
| `server/src/project_handlers.rs` | Evaluate handler — add expression evaluation branch, include expression in BKM response |
| `type-registry/src/tests/fixtures/commercial_lending/decisions/ltv_calculation.md` | LTV BKM — add `feel-expression` and `output-name` |
| `type-registry/src/tests/fixtures/commercial_lending/decisions/dscr_calculation.md` | DSCR BKM — add `feel-expression` and `output-name` |
| `type-registry/src/tests/fixtures/commercial_lending/decisions/collateral_adequacy.md` | Fix hit policy U → F |
| `type-registry/src/tests/fixtures/commercial_lending/types/inputs.ts` | Add `Annual Debt Service` field |
| `type-registry/src/tests/fixtures/commercial_lending/scenarios.json` | Add `Annual Debt Service` to all scenarios |
| `model-evaluator/src/tests/markdown_eval.rs` | Regression tests for FEEL expression evaluation |
| `viewer/src/lib/types.ts` | Add `feel_expression` to BKM type |
| `viewer/src/lib/components/ProjectBkmNode.svelte` | Render expression text for expression BKMs |

---

### Task 1: Add `feel-expression` and `output-name` to front matter parser

**Files:**
- Modify: `type-registry/src/front_matter.rs:17-33`

- [ ] **Step 1: Add the new fields to DmnNode**

```rust
// In type-registry/src/front_matter.rs, add two fields to DmnNode:

#[derive(Debug, Clone, Deserialize)]
pub struct DmnNode {
  pub id: String,
  #[serde(rename = "type")]
  pub node_type: String,
  pub name: String,
  #[serde(rename = "data-type")]
  pub data_type: Option<DataTypeRef>,
  pub signature: Option<BkmSignature>,
  pub uri: Option<String>,
  pub owner: Option<String>,
  pub requires: Option<Vec<String>>,
  #[serde(rename = "governed-by")]
  pub governed_by: Option<Vec<String>>,
  #[serde(rename = "supported-by")]
  pub supported_by: Option<Vec<String>>,
  #[serde(rename = "feel-expression")]
  pub feel_expression: Option<String>,
  #[serde(rename = "output-name")]
  pub output_name: Option<String>,
}
```

- [ ] **Step 2: Verify existing tests still pass**

Run: `cargo +stable test -p dsntk-type-registry`
Expected: All existing tests pass (the new fields are `Option`, so existing front matter without them still parses).

- [ ] **Step 3: Commit**

```bash
git add type-registry/src/front_matter.rs
git commit -m "feat(type-registry): add feel-expression and output-name to front matter"
```

---

### Task 2: Update LTV and DSCR markdown files

**Files:**
- Modify: `type-registry/src/tests/fixtures/commercial_lending/decisions/ltv_calculation.md`
- Modify: `type-registry/src/tests/fixtures/commercial_lending/decisions/dscr_calculation.md`

- [ ] **Step 1: Update ltv_calculation.md front matter**

Replace the entire front matter block with:

```yaml
---
dmn:
  id: ltv_calculation
  type: bkm
  name: LTV Calculation
  feel-expression: Requested Amount / Valuation Amount
  output-name: LTV
  requires:
    - loan_request
    - property_data
---
```

Keep the existing markdown body (documentation) unchanged.

- [ ] **Step 2: Update dscr_calculation.md front matter**

Replace the entire front matter block with:

```yaml
---
dmn:
  id: dscr_calculation
  type: bkm
  name: DSCR Calculation
  feel-expression: (Net Profit + Depreciation + Interest Expense) / Annual Debt Service
  output-name: DSCR
  requires:
    - financial_statements
    - loan_request
---
```

Keep the existing markdown body unchanged.

- [ ] **Step 3: Verify front matter parsing**

Run: `cargo +stable test -p dsntk-type-registry`
Expected: All tests pass. The scanner reads these files and the new fields are now populated.

- [ ] **Step 4: Commit**

```bash
git add type-registry/src/tests/fixtures/commercial_lending/decisions/ltv_calculation.md \
        type-registry/src/tests/fixtures/commercial_lending/decisions/dscr_calculation.md
git commit -m "feat: add feel-expression and output-name to LTV and DSCR BKMs"
```

---

### Task 3: Add "Annual Debt Service" to types and scenarios

**Files:**
- Modify: `type-registry/src/tests/fixtures/commercial_lending/types/inputs.ts`
- Modify: `type-registry/src/tests/fixtures/commercial_lending/scenarios.json`

- [ ] **Step 1: Add field to FinancialStatements interface**

In `inputs.ts`, add to the `FinancialStatements` interface:

```typescript
export interface FinancialStatements {
  "Net Profit": number;
  "Depreciation": number;
  "Interest Expense": number;
  "Total Debt": number;
  "Total Assets": number;
  "Accounts Type": string;
  "Accounts Age": number;
  "Leverage Ratio": number;
  "Annual Debt Service": number;
}
```

- [ ] **Step 2: Add values to all 5 scenarios in scenarios.json**

Add `"Annual Debt Service"` to each scenario's `financial_statements` object:

| Scenario | Value |
|----------|-------|
| Strong Applicant (Nexus Solutions) | 70000 |
| Marginal (Harbour View Hotel) | 50000 |
| Decline (QuickBuild Contractors) | 50000 |
| Edge Case (MedPark Group) | 200000 |
| Sole Trader (Sarah's Florist) | 8333 |

Each entry goes after `"Leverage Ratio"` in the `financial_statements` block.

For example, the Strong Applicant scenario's financial_statements becomes:

```json
"financial_statements": {
  "Net Profit": 680000,
  "Depreciation": 45000,
  "Interest Expense": 32000,
  "Total Debt": 250000,
  "Total Assets": 2800000,
  "Accounts Type": "Audited",
  "Accounts Age": 4,
  "Leverage Ratio": 0.089,
  "Annual Debt Service": 70000
}
```

- [ ] **Step 3: Commit**

```bash
git add type-registry/src/tests/fixtures/commercial_lending/types/inputs.ts \
        type-registry/src/tests/fixtures/commercial_lending/scenarios.json
git commit -m "feat: add Annual Debt Service to input types and scenarios"
```

---

### Task 4: Fix Collateral Adequacy hit policy

**Files:**
- Modify: `type-registry/src/tests/fixtures/commercial_lending/decisions/collateral_adequacy.md`

- [ ] **Step 1: Change hit policy from U to F**

In `collateral_adequacy.md`, change the table header row:

From:
```
| U  | LTV          | Property Risk    | Purpose            | Collateral Grade                                 |
```

To:
```
| F  | LTV          | Property Risk    | Purpose            | Collateral Grade                                 |
```

- [ ] **Step 2: Commit**

```bash
git add type-registry/src/tests/fixtures/commercial_lending/decisions/collateral_adequacy.md
git commit -m "fix: change Collateral Adequacy hit policy from UNIQUE to FIRST"
```

---

### Task 5: Add FEEL expression evaluation to the server handler

**Files:**
- Modify: `server/src/project_handlers.rs`

- [ ] **Step 1: Add expression evaluation branch**

In the `evaluate_project` handler's loop over `evaluation_order`, replace the existing `"decision" | "bkm"` match arm. The new logic checks `feel_expression` first:

```rust
"decision" | "bkm" => {
  let Ok(content) = std::fs::read_to_string(&drg_node.file_path) else {
    continue;
  };

  // Check if this node has a FEEL expression (expression-only BKM).
  let (eval_value, matched_rules, cell_evals) = if let Some(ref feel_expr) = drg_node.dmn.feel_expression {
    match dsntk_feel_parser::parse_expression(&scope, feel_expr, false) {
      Ok(ast_node) => {
        let evaluator = dsntk_feel_evaluator::prepare(&ast_node);
        let value = evaluator(&scope);
        (value, vec![], vec![])
      }
      Err(e) => (Value::Null(Some(format!("{}", e))), vec![], vec![]),
    }
  } else {
    // Decision table evaluation (existing logic).
    let Some(md_body) = extract_body(&content) else {
      continue;
    };
    let Ok(recognized_dt) = dsntk_recognizer::from_markdown(md_body, false) else {
      continue;
    };
    let model_dt: dsntk_model::DecisionTable = recognized_dt.into();
    match evaluate_decision_table_with_trace(&scope, &model_dt) {
      Ok(r) => {
        let ce: Vec<serde_json::Value> = r.cell_evaluations.iter().map(|ce| serde_json::to_value(ce).unwrap_or_default()).collect();
        (r.value, r.matched_rules, ce)
      }
      Err(e) => (Value::Null(Some(format!("{}", e))), vec![], vec![]),
    }
  };

  // Store the result in scope so downstream decisions can use it.
  let name = Name::new(&[&drg_node.dmn.name]);
  if let Some(mut top_ctx) = scope.pop() {
    top_ctx.set_entry(&name, eval_value.clone());

    // Store under output-name if specified (for expression BKMs like "LTV", "DSCR").
    if let Some(ref out_name) = drg_node.dmn.output_name {
      let output_key = Name::new(&[out_name]);
      top_ctx.set_entry(&output_key, eval_value.clone());
    }

    // For decision tables, store under output column names.
    if drg_node.dmn.feel_expression.is_none() {
      let Ok(recognized_dt) = dsntk_recognizer::from_markdown(
        extract_body(&content).unwrap_or(""),
        false,
      ) else {
        scope.push(top_ctx);
        // still push result even if re-parse fails
        results.push(NodeEvalResult {
          node_id: node_id.clone(),
          node_name: drg_node.dmn.name.clone(),
          matched_rules: matched_rules.clone(),
          output_value: serde_json::to_value(eval_value.jsonify()).unwrap_or(serde_json::Value::Null),
          cell_evaluations: cell_evals,
        });
        continue;
      };
      let model_dt: dsntk_model::DecisionTable = recognized_dt.into();
      let output_clauses: Vec<_> = model_dt.output_clauses().collect();
      if output_clauses.len() == 1 {
        if let Some(ref output_name) = output_clauses[0].name {
          let out_name = Name::new(&[output_name]);
          top_ctx.set_entry(&out_name, eval_value.clone());
        }
      } else {
        if let Value::Context(ref out_ctx) = eval_value {
          for (entry_name, entry_value) in out_ctx.iter() {
            top_ctx.set_entry(entry_name, entry_value.clone());
          }
        }
      }
    }

    scope.push(top_ctx);
  }

  let output_json = serde_json::to_value(eval_value.jsonify()).unwrap_or(serde_json::Value::Null);
  results.push(NodeEvalResult {
    node_id: node_id.clone(),
    node_name: drg_node.dmn.name.clone(),
    matched_rules,
    output_value: output_json,
    cell_evaluations: cell_evals,
  });
}
```

- [ ] **Step 2: Add imports**

Add to the imports at the top of `project_handlers.rs`:

```rust
use dsntk_feel_evaluator;
use dsntk_feel_parser;
```

- [ ] **Step 3: Add feel-evaluator and feel-parser to server's Cargo.toml dependencies**

In `server/Cargo.toml` under `[dependencies]`, add:

```toml
dsntk-feel-evaluator = { path = "../feel-evaluator" }
dsntk-feel-parser = { path = "../feel-parser" }
```

- [ ] **Step 4: Include `feel_expression` in BKM node response**

In the `load_project` handler's BKM branch (around line 178), add the expression to the response:

In the `ProjectNode::Bkm` enum variant, add a new field:

```rust
#[serde(rename = "bkm")]
Bkm {
  id: String,
  name: String,
  hit_policy: String,
  input_columns: Vec<String>,
  output_columns: Vec<String>,
  rules: Vec<ProjectRule>,
  parameters: Vec<ProjectParam>,
  #[serde(skip_serializing_if = "Option::is_none")]
  feel_expression: Option<String>,
},
```

In the BKM branch of the `load_project` handler, populate it:

```rust
"bkm" => {
  let (hp, ic, oc, rules, _ac) = parse_decision_table_from_file(&drg_node.file_path);
  let parameters = dmn
    .signature
    .as_ref()
    .map(|sig| sig.parameters.iter().map(|p| ProjectParam { name: p.name.clone(), param_type: p.param_type.clone() }).collect())
    .unwrap_or_default();
  nodes.push(ProjectNode::Bkm {
    id: dmn.id.clone(),
    name: dmn.name.clone(),
    hit_policy: hp,
    input_columns: ic,
    output_columns: oc,
    rules,
    parameters,
    feel_expression: dmn.feel_expression.clone(),
  });
}
```

- [ ] **Step 5: Build and verify**

Run: `cargo +stable build --workspace`
Expected: Compiles successfully.

- [ ] **Step 6: Commit**

```bash
git add server/src/project_handlers.rs server/Cargo.toml
git commit -m "feat(server): evaluate FEEL expression BKMs and store results in scope"
```

---

### Task 6: Add regression tests for FEEL expression evaluation

**Files:**
- Modify: `model-evaluator/src/tests/markdown_eval.rs`

- [ ] **Step 1: Add FEEL expression evaluation tests**

Append to `model-evaluator/src/tests/markdown_eval.rs`:

```rust
// ---------------------------------------------------------------------------
// FEEL expression evaluation (not decision tables)
// ---------------------------------------------------------------------------

/// FEEL expression: simple division with multi-word names in scope.
#[test]
fn _0011() {
  let scope = scope_with(&[
    ("Requested Amount", num("350000")),
    ("Valuation Amount", num("750000")),
  ]);
  let ast = dsntk_feel_parser::parse_expression(&scope, "Requested Amount / Valuation Amount", false)
    .expect("failed to parse expression");
  let evaluator = dsntk_feel_evaluator::prepare(&ast);
  let result = evaluator(&scope);
  // 350000 / 750000 = 0.466666...
  match &result {
    Value::Number(n) => {
      let expected: dsntk_feel::FeelNumber = "0.4666666666666666666666666666666667".parse().unwrap();
      assert_eq!(*n, expected);
    }
    other => panic!("expected number, got: {other}"),
  }
}

/// FEEL expression: DSCR formula (net profit + depreciation + interest) / debt service.
#[test]
fn _0012() {
  let scope = scope_with(&[
    ("Net Profit", num("680000")),
    ("Depreciation", num("45000")),
    ("Interest Expense", num("32000")),
    ("Annual Debt Service", num("70000")),
  ]);
  let ast = dsntk_feel_parser::parse_expression(
    &scope,
    "(Net Profit + Depreciation + Interest Expense) / Annual Debt Service",
    false,
  )
  .expect("failed to parse DSCR expression");
  let evaluator = dsntk_feel_evaluator::prepare(&ast);
  let result = evaluator(&scope);
  // (680000 + 45000 + 32000) / 70000 = 757000 / 70000 = 10.81428...
  match &result {
    Value::Number(_) => {} // Just verify it's a number, not null
    other => panic!("expected number, got: {other}"),
  }
}

/// FEEL expression: division by zero returns null.
#[test]
fn _0013() {
  let scope = scope_with(&[
    ("Requested Amount", num("350000")),
    ("Valuation Amount", num("0")),
  ]);
  let ast = dsntk_feel_parser::parse_expression(&scope, "Requested Amount / Valuation Amount", false)
    .expect("failed to parse expression");
  let evaluator = dsntk_feel_evaluator::prepare(&ast);
  let result = evaluator(&scope);
  // Division by zero in FEEL produces null
  match &result {
    Value::Null(_) => {}
    other => panic!("expected null for division by zero, got: {other}"),
  }
}

/// FEEL expression: missing name in scope produces null.
#[test]
fn _0014() {
  let scope = scope_with(&[("Requested Amount", num("350000"))]);
  // "Valuation Amount" is NOT in scope
  let ast = dsntk_feel_parser::parse_expression(&scope, "Requested Amount / Valuation Amount", false);
  // The parser may fail or the evaluator may produce null — either is acceptable.
  match ast {
    Ok(node) => {
      let evaluator = dsntk_feel_evaluator::prepare(&node);
      let result = evaluator(&scope);
      match &result {
        Value::Null(_) => {}
        other => panic!("expected null for missing name, got: {other}"),
      }
    }
    Err(_) => {} // Parser error is also acceptable
  }
}
```

- [ ] **Step 2: Run the new tests**

Run: `cargo +stable test -p dsntk-model-evaluator -- markdown_eval`
Expected: All 14 tests pass (10 existing + 4 new).

- [ ] **Step 3: Commit**

```bash
git add model-evaluator/src/tests/markdown_eval.rs
git commit -m "test: add regression tests for FEEL expression evaluation with multi-word names"
```

---

### Task 7: Update BKM node frontend display

**Files:**
- Modify: `viewer/src/lib/types.ts`
- Modify: `viewer/src/lib/components/ProjectBkmNode.svelte`

- [ ] **Step 1: Add feel_expression to the TypeScript BKM type**

In `viewer/src/lib/types.ts`, find the `ProjectNode` union type's BKM variant and add `feel_expression`:

```typescript
// In the BKM variant of ProjectNode:
{
  type: 'bkm';
  id: string;
  name: string;
  hit_policy: string;
  input_columns: string[];
  output_columns: string[];
  rules: ProjectRule[];
  parameters: ProjectParam[];
  feel_expression?: string;
}
```

- [ ] **Step 2: Update ProjectBkmNode.svelte to show expression**

In `viewer/src/lib/components/ProjectBkmNode.svelte`, update the `data` type and add expression display.

Update the `data` type in the props:

```typescript
let { id, data }: {
  id: string;
  data: {
    name: string;
    hit_policy: string;
    input_columns: string[];
    output_columns: string[];
    rules: ProjectRule[];
    parameters: ProjectParam[];
    feel_expression?: string;
  };
} = $props();
```

After the `{#if data.parameters.length > 0}` block and before `{#if data.rules.length > 0}`, add:

```svelte
{#if data.feel_expression}
  <div class="expression">
    <span class="expr-label">FEEL</span>
    <code class="expr-code">{data.feel_expression}</code>
  </div>
{/if}
```

Add styles inside the `<style>` block:

```css
.expression {
  padding: 6px 12px; display: flex; gap: 8px; align-items: center;
  border-bottom: 1px solid #30363d;
}
.expr-label {
  background: #1f3a5f; color: #58a6ff; padding: 2px 6px;
  border-radius: 4px; font-size: 9px; font-weight: bold; white-space: nowrap;
}
.expr-code { color: #c9d1d9; font-size: 10px; }
.evaluated .expr-code { color: #3fb950; }
```

And modify the rules block to only show when there's no expression:

```svelte
{#if !data.feel_expression && data.rules.length > 0}
```

(Change `{#if data.rules.length > 0}` to `{#if !data.feel_expression && data.rules.length > 0}`)

- [ ] **Step 3: Verify frontend builds**

Run: `cd viewer && npm run build`
Expected: Build succeeds with no errors.

- [ ] **Step 4: Commit**

```bash
git add viewer/src/lib/types.ts viewer/src/lib/components/ProjectBkmNode.svelte
git commit -m "feat(viewer): display FEEL expression in BKM nodes"
```

---

### Task 8: End-to-end verification

- [ ] **Step 1: Build everything**

Run: `cargo +stable build --workspace`
Expected: Clean build.

- [ ] **Step 2: Run all Rust tests**

Run: `cargo +stable test --workspace`
Expected: All tests pass.

- [ ] **Step 3: Start servers and test evaluate endpoint**

Start backend:
```bash
nohup ./target/debug/dsntk srv -H 127.0.0.1 -P 22022 > /tmp/dsntk-srv.log 2>&1 &
```

Wait for startup, then test the Strong Applicant scenario:

```bash
curl -s -X POST http://127.0.0.1:22022/api/v1/project/evaluate \
  -H "Content-Type: application/json" \
  -d '{"dir":"type-registry/src/tests/fixtures/commercial_lending","inputs":{...Strong Applicant data with Annual Debt Service: 70000...}}'
```

Expected results for Strong Applicant:
- **LTV Calculation**: ~0.467 (350000 / 750000)
- **DSCR Calculation**: ~10.81 ((680000+45000+32000) / 70000)
- **Credit Score Classification**: "A"
- **Property Risk Rating**: "Low"
- **Sector Risk Assessment**: "Low"
- **Accounts Quality Check**: "High"
- **Collateral Adequacy**: "Strong" (LTV ~0.47, Property Risk "Low")
- **Pricing Tier**: "T1" (Grade A, Strong collateral, high DSCR, Low sector risk, High accounts quality)
- **Rate Adjustments**: GTR-01 (guarantee discount)
- **Final Approval**: "Approved" (Auto)
- **Conditions of Approval**: fee + annual review conditions

- [ ] **Step 4: Verify in browser**

Open the viewer, load the commercial lending project, click "Strong Applicant" scenario, click Evaluate. All nodes should show green highlighting with results.

- [ ] **Step 5: Commit and push**

```bash
git push
```
