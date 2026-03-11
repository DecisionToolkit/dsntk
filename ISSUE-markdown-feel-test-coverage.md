# Improve test coverage for markdown decision tables with FEEL expressions

## Summary

The markdown decision table feature (parsing markdown tables as DMN decision tables and evaluating FEEL expressions within them) currently has minimal test coverage. All existing markdown BBT tests use the **same simple discount-lookup pattern** with hit policy `U`, string equality inputs, and numeric comparisons. This issue proposes comprehensive additional tests to cover real-world usage patterns.

## Current State

**Existing markdown decision table tests (BBT):**
- `cli/edt/ok_markdown` — Evaluate a simple 3-rule discount table (U, strings + `<10`/`>=10`, numeric output)
- `cli/pdt/ok_markdown` — Parse the same table
- `cli/pdt/ok_markdown_full_table` — Parse a table with annotations, allowed values, and `> #` information item name
- `cli/tdt/ok_markdown` — Test the same discount table with 8 input/expected pairs
- `cli/tdt/ok_markdown_test_summary` — Same table with summary output
- `cli/xdt/ok_markdown` — Explain the same discount table

**Unit tests in `recognizer/src/markdown.rs`:**
- `test_is_monotonic` — monotonic number detection
- `test_strip_emphasis` — markdown emphasis stripping
- `test_get_marker` — marker string recognition

**What's well-covered elsewhere:**
- FEEL expression parsing and evaluation (200+ test modules in `feel-parser` and `feel-evaluator`)
- DMN model evaluation via XML (150+ compatibility tests)
- Exhaustive decision table tests via DMN XML (multiple default/no-default scenarios)

**Key gap:** The integration of markdown tables **with** diverse FEEL expressions is barely tested. The markdown parser is tested for structure, and FEEL is tested for expressions, but end-to-end coverage of real decision tables authored in markdown is thin.

## Recommended Tests

### 1. Hit Policy Coverage in Markdown

Currently only `U` (Unique) is tested. Add markdown tables for:

- [ ] **`A` (Any)** — Multiple rules can match but must produce the same output
- [ ] **`F` (First)** — First matching rule wins (rule order matters)
- [ ] **`P` (Priority)** — Output priority determines winner among matches
- [ ] **`C` (Collect)** — Collect all matching outputs into a list
- [ ] **`C+` (Collect Sum)** — Sum of all matching output values
- [ ] **`C#` (Collect Count)** — Count of matching rules
- [ ] **`C<` (Collect Min)** — Minimum of matching outputs
- [ ] **`C>` (Collect Max)** — Maximum of matching outputs

**Example — Collect Sum (`C+`) for tax brackets:**
```markdown
| C+ | Taxable Income | Tax Rate |
|:--:|:--------------:|:--------:|
|    |     `in`       |  `out`   |
| 1  |   [0..10000]   |   0.10   |
| 2  | (10000..50000] |   0.22   |
| 3  | (50000..100000]|   0.32   |
| 4  |    >100000     |   0.37   |
```

### 2. FEEL Expression Types in Input Entries

Currently only string equality (`"Business"`) and simple numeric comparisons (`<10`, `>=10`) are tested. Add:

- [ ] **Range expressions** — `[18..65]`, `(0..100)`, `[1..10)`
- [ ] **Negation** — `not("Cancelled")`, `not(0)`
- [ ] **Disjunction in entries** — `"Gold","Platinum"` (comma-separated values)
- [ ] **Null checks** — explicit `null` input entries
- [ ] **Boolean inputs** — `true`, `false` as input values
- [ ] **Date comparisons** — `>date("2025-01-01")`, `[date("2024-01-01")..date("2024-12-31")]`
- [ ] **Duration comparisons** — `>duration("P1Y")`, `<=duration("P30D")`
- [ ] **Wildcard `-`** for "any value" (tested but only in one position)

### 3. FEEL Expression Types in Output Entries

Currently only numeric outputs are tested. Add:

- [ ] **String outputs** — `"Approved"`, `"Denied"`
- [ ] **Boolean outputs** — `true`, `false`
- [ ] **Date outputs** — `date("2025-06-01")`
- [ ] **Duration outputs** — `duration("P30D")`
- [ ] **List outputs** — `["email","sms"]`
- [ ] **Context outputs** — `{rate: 0.05, cap: 1000}`
- [ ] **Null output** — explicit `null`
- [ ] **Arithmetic expressions** — `Salary * 0.05`, `Price * (1 - Discount)`

### 4. Multiple Inputs and Outputs

Current tests use at most 2 inputs and 1-2 outputs. Add:

- [ ] **3+ input columns** — e.g., Age, Income, Credit Score -> Approval, Rate
- [ ] **3+ output columns** — e.g., Decision, Reason, Next Step
- [ ] **Single input, multiple outputs** — routing/classification scenarios

### 5. Vertical (Rules-as-Columns) Orientation

No markdown BBT tests currently cover vertical tables:

- [ ] **Basic vertical table** — same discount logic but transposed
- [ ] **Vertical with allowed values**
- [ ] **Vertical with annotations**

### 6. Allowed Values and Default Outputs

Only one test (`ok_markdown_full_table`) covers these features, and it's only parsed, not evaluated:

- [ ] **Evaluate a table with allowed values** — verify rejection of out-of-range inputs
- [ ] **Default output values** — verify `*` emphasis syntax works (e.g., `*"Normal"*` as default)
- [ ] **Multiple outputs with different defaults**

### 7. Real-World Business Scenarios

These test complete decision-making patterns that users would actually author in markdown:

- [ ] **Loan eligibility** — Credit score ranges, income thresholds, employment status → Approval + Interest rate

  ```markdown
  | F | Credit Score | Annual Income | Employment |  Decision  |  Rate  |
  |:-:|:------------:|:-------------:|:----------:|:----------:|:------:|
  |   |    `in`      |     `in`      |    `in`    |   `out`    | `out`  |
  | 1 |   >=750      |   >=50000     | "Employed" | "Approved" |  3.5   |
  | 2 |  [700..750)  |   >=40000     | "Employed" | "Approved" |  5.0   |
  | 3 |  [650..700)  |   >=60000     | "Employed" | "Review"   |  7.5   |
  | 4 |    <650      |       -       |     -      | "Denied"   | null   |
  | 5 |      -       |       -       | "Unemployed"| "Denied"  | null   |
  ```

- [ ] **Shipping cost calculation** — Weight ranges, destination zones, express flag → Cost
- [ ] **Insurance premium** — Age ranges, smoker boolean, coverage level → Monthly premium (using `C+` for additive surcharges)
- [ ] **SLA response time** — Priority + Customer tier → Response hours (using durations)
- [ ] **Discount stacking** — Customer type + Order value + Membership duration → Discount percentage

### 8. Edge Cases and Error Handling

- [ ] **Single-rule table** — only one data row
- [ ] **Many rules (20+)** — verify no performance or parsing issues
- [ ] **Empty/whitespace in cells** — verify graceful handling
- [ ] **Special characters in strings** — quotes, pipes, backticks in string values
- [ ] **Very long FEEL expressions** — complex nested expressions in cells
- [ ] **Mismatched column counts** — verify proper error reporting
- [ ] **Missing hit policy** — verify proper error
- [ ] **Invalid marker combinations** — e.g., output before input

### 9. Markdown Formatting Variations

- [ ] **No alignment colons** — `|---|` instead of `|:-:|`
- [ ] **Left/right alignment** — `|:--|` and `|--:|`
- [ ] **Extra whitespace** — varying amounts of padding in cells
- [ ] **Surrounding markdown content** — table embedded in paragraphs, headers, lists
- [ ] **All marker synonyms in context** — `>>>`, `>>`, `<<<`, `<<`, `#`, `##`, `###` used in actual tables

### 10. FEEL Built-in Functions in Table Entries

- [ ] **String functions** — `contains(Description, "urgent")`, `upper case(Status)`
- [ ] **Numeric functions** — `abs(Balance)`, `ceiling(Score / 10)`
- [ ] **List functions** — `count(Items) > 5`, `list contains(Tags, "priority")`
- [ ] **Temporal functions** — `today() - Application Date > duration("P30D")`

## Priority

I'd suggest implementing these in this order:
1. **Hit policy coverage** (#1) — fundamental feature, easy to add
2. **Real-world scenarios** (#7) — highest value for validating end-to-end correctness
3. **FEEL input/output types** (#2, #3) — broadens expression coverage
4. **Allowed values & defaults** (#6) — important feature with minimal coverage
5. **Vertical orientation** (#5) — alternative layout
6. **Edge cases** (#8) — robustness
7. **Formatting variations** (#9) — resilience to real markdown
8. **Built-in functions in entries** (#10) — advanced usage
9. **Multiple inputs/outputs** (#4) — scaling verification
