# DMN Evaluation Viewer — Design Spec

**Date:** 2026-04-08
**Status:** Draft
**Goal:** Make DMN rule evaluation explainable by visualizing the decision requirements graph (DRG) and highlighting the evaluation path through it.

## Overview

A SvelteKit web application (`viewer/`) that visualizes DMN model evaluation as an interactive, animated directed acyclic graph (DAG). Users enter input values, trigger evaluation against the dsntk REST server, and watch decisions propagate through the graph with full per-rule transparency. The target audience is domain experts, auditors, and developers who need to understand *why* a decision was reached — making the AI (expert system) explainable.

## Architecture

**Approach:** New `/api/v1/evaluate-trace` endpoint on the existing dsntk Actix-web server, consumed by a SvelteKit frontend.

```
┌─────────────┐       POST /evaluate-trace       ┌──────────────────┐
│  SvelteKit  │ ──────────────────────────────▶  │  dsntk-server    │
│  viewer/    │ ◀──────────────────────────────  │  (Actix-web)     │
│             │       JSON trace response         │                  │
│  Port 5173  │                                   │  Port 8080       │
│  (dev)      │       GET /models                 │                  │
│             │ ──────────────────────────────▶  │                  │
└─────────────┘                                   └──────────────────┘
```

## Layout

Three-panel layout:

### Left Panel — Input Panel
- **Model selector:** Dropdown to choose the DMN model and invocable (decision service or top-level decision).
- **Auto-generated input fields:** Driven by `TypeRegistry` metadata from the backend:
  - `FeelType::Number` → numeric input
  - `FeelType::String` with `allowed_values` → dropdown select
  - `FeelType::String` without `allowed_values` → text input
  - `FeelType::Boolean` → toggle switch
  - `FeelType::Date` / `DateTime` / `Time` → date/time pickers
  - Fields marked `optional` in TypeRegistry can be left blank
- **Evaluate button:** Triggers `POST /api/v1/evaluate-trace` and populates the canvas.

### Center Panel — DAG Canvas
- **Svelte Flow** (`@xyflow/svelte`) canvas with pan, zoom, and minimap.
- **Auto-layout** via `dagre` — top-to-bottom flow, inputs at top, final decision at bottom.
- **Step-through controls** (top-left toolbar):
  - Skip to start / Step back / Play-Pause / Step forward / Skip to end
  - Step counter: "Step N / M"
  - Speed slider for animation playback

### Node Visual States
Three states per node, applied progressively during step-through:

| State | Border | Background | Content |
|---|---|---|---|
| Unevaluated | Grey, dashed | Dark (default) | Table visible, all rows grey |
| Evaluating (current step) | Yellow, pulsing | Dark | Yellow highlight on current step |
| Evaluated | Dark green, solid | Light green on matched rows | Matched rows green, unmatched dimmed |

### Edge Visual States

| State | Appearance |
|---|---|
| Inactive | Grey, dashed line |
| Animating (current step) | Animated pulse (particle) traveling source → target |
| Completed | Solid green line + value label (e.g. `→ "Eligible"`) |

## Node Types

### Input Data Node
- Rounded rectangle with blue accent border
- Shows: input name + current value
- Example: `Applicant Age: 25`

### Decision Table Node
Full decision table rendered inside the node:
- **Header:** Decision name + hit policy badge (e.g. `U (Unique)`)
- **Column headers:** Input column names | Output column names
- **Rule rows:** All rows always visible. Matched rows get dark green border-left + light green background. Unmatched rows are dimmed grey.
- **Result footer:** "Rule N matched → output_value"

All rows are always visible (no collapsing). The goal is full transparency.

## Interaction Modes

### 1. Animated Step-Through
After evaluation, the user can step through the evaluation in topological order:
- Each step highlights the current decision node (yellow → green), animates incoming edges, and reveals matched rules.
- "Play" auto-advances through steps at a configurable speed.
- "Step back" reverses to the previous state (purely a frontend concern — replaying from trace data).

### 2. Live Re-evaluation
When the user modifies input values in the side panel:
- A debounced re-evaluation call fires automatically.
- The DAG re-renders with the new trace, resetting to the fully-evaluated state (all steps complete).
- The user can then step through the new evaluation.

## API Contract

### `GET /api/v1/models`

Returns available models with their input metadata (driven by `TypeRegistry`).

```json
{
  "models": [
    {
      "namespace": "https://example.com/loan",
      "name": "Loan Approval",
      "invocables": ["Loan Decision"],
      "inputs": [
        {
          "name": "Applicant Age",
          "feel_type": "number",
          "allowed_values": null,
          "optional": false
        },
        {
          "name": "Employment Status",
          "feel_type": "string",
          "allowed_values": ["Employed", "Unemployed", "Self-Employed"],
          "optional": false
        }
      ]
    }
  ]
}
```

### `POST /api/v1/evaluate-trace`

**Request:**
```json
{
  "namespace": "https://example.com/loan",
  "invocable": "Loan Decision",
  "inputs": {
    "Applicant Age": 25,
    "Annual Income": 55000,
    "Credit Score": 720,
    "Employment Status": "Employed"
  }
}
```

**Response:**
```json
{
  "trace": {
    "graph": {
      "nodes": [
        {
          "id": "input_applicant_age",
          "type": "input_data",
          "name": "Applicant Age",
          "value": 25
        },
        {
          "id": "dt_eligibility_check",
          "type": "decision_table",
          "name": "Eligibility Check",
          "hit_policy": "Unique",
          "input_columns": ["Age", "Income"],
          "output_columns": ["Result"],
          "rules": [
            {
              "index": 0,
              "input_entries": [">=18", ">30000"],
              "output_entries": ["\"Eligible\""]
            },
            {
              "index": 1,
              "input_entries": [">=18", "<=30000"],
              "output_entries": ["\"Review\""]
            },
            {
              "index": 2,
              "input_entries": ["<18", "-"],
              "output_entries": ["\"Rejected\""]
            }
          ]
        }
      ],
      "edges": [
        {
          "source": "input_applicant_age",
          "target": "dt_eligibility_check",
          "label": "Applicant Age"
        }
      ]
    },
    "evaluation_order": [
      "dt_eligibility_check",
      "dt_risk_assessment",
      "dt_loan_decision"
    ],
    "steps": [
      {
        "node_id": "dt_eligibility_check",
        "input_values": { "Age": 25, "Income": 55000 },
        "matched_rules": [0],
        "output_value": "Eligible",
        "cell_evaluations": [
          {
            "rule_index": 0,
            "column_index": 0,
            "expression": ">=18",
            "input_value": 25,
            "result": true
          },
          {
            "rule_index": 0,
            "column_index": 1,
            "expression": ">30000",
            "input_value": 55000,
            "result": true
          },
          {
            "rule_index": 1,
            "column_index": 0,
            "expression": ">=18",
            "input_value": 25,
            "result": true
          },
          {
            "rule_index": 1,
            "column_index": 1,
            "expression": "<=30000",
            "input_value": 55000,
            "result": false
          }
        ]
      }
    ]
  }
}
```

## Backend Changes (Rust)

### New: `model-evaluator/src/trace.rs`

Trace data structures, all `#[derive(Debug, Clone, Serialize)]`:

- **`EvaluationTrace`** — top-level: `graph: TraceGraph`, `evaluation_order: Vec<String>`, `steps: Vec<TraceStep>`
- **`TraceGraph`** — `nodes: Vec<TraceNode>`, `edges: Vec<TraceEdge>`
- **`TraceNode`** — enum: `InputData { id, name, value }` or `DecisionTable { id, name, hit_policy, input_columns, output_columns, rules }`
- **`TraceEdge`** — `source: String`, `target: String`, `label: String`
- **`TraceStep`** — `node_id`, `input_values`, `matched_rules: Vec<usize>`, `output_value`, `cell_evaluations: Vec<CellEvaluation>`
- **`CellEvaluation`** — `rule_index`, `column_index`, `expression: String`, `input_value` (JSON value), `result` (bool or JSON value). Captured for *all* rules (not just matched ones) to provide a complete audit trail.

### Integration: `model-evaluator/src/decision.rs`

During recursive decision evaluation, when trace collection is enabled:
1. After resolving requirements and evaluating decision logic, push a `TraceStep` with the results.
2. The depth-first evaluation order naturally produces the correct topological sequence.

### Integration: `model-evaluator/src/decision_table.rs`

During rule evaluation (where `EvaluatedRule` structs are already built):
1. For each rule, for each input cell, capture the expression text, the input value, and the boolean result into `CellEvaluation`.
2. This data is already computed internally — we record it rather than discarding it.

### Graph Extraction: `model-evaluator/src/model_definitions.rs`

Extract DAG topology from `DefDefinitions`:
- Each `DefDecision` → `TraceNode::DecisionTable` (with table structure from the parsed `DecisionTable`)
- Each `DefInputData` → `TraceNode::InputData`
- Each `InformationRequirement` → `TraceEdge`

### Input Metadata: `type-registry/src/registry.rs`

The `TypeRegistry` provides:
- `TypeEntry.feel_type` → maps to frontend input widget type
- `TypeEntry.allowed_values` → dropdown options for enum/union types
- `TypeEntry.optional_fields` → which inputs can be omitted

### New Endpoints: `server/src/`

Two new Actix-web handlers under `/api/v1/`:
- `GET /models` — iterates workspace models, extracts input metadata from TypeRegistry, returns JSON
- `POST /evaluate-trace` — deserializes input, calls evaluation pipeline with trace enabled, returns trace JSON

### CORS

Add `actix-cors` dependency for development (SvelteKit dev server on different port). Configure to allow `localhost` origins.

## Frontend: `viewer/`

### Tech Stack
- **SvelteKit** — app framework
- **@xyflow/svelte** (Svelte Flow) — DAG canvas
- **dagre** — automatic graph layout (top-to-bottom)
- **TypeScript** — throughout

### Components

| Component | Purpose |
|---|---|
| `+page.svelte` | Top-level three-panel layout |
| `InputPanel.svelte` | Model selector, auto-generated inputs, evaluate button |
| `DagCanvas.svelte` | Svelte Flow wrapper, dagre layout, zoom/pan, minimap |
| `StepControls.svelte` | Play/pause, step forward/back, speed, step counter |
| `DecisionTableNode.svelte` | Custom node: full table with matched-row highlighting |
| `InputDataNode.svelte` | Custom node: input name + value display |
| `AnimatedEdge.svelte` | Custom edge: pulse animation + value label |

### Svelte Stores

| Store | Contents |
|---|---|
| `traceStore` | Full API response from `/evaluate-trace` |
| `stepStore` | Current step index (0 to N), controls animation state |
| `inputStore` | Current input values, triggers debounced re-evaluation |

### Evaluation Flow

1. User selects model → `GET /models` populates input panel
2. User fills inputs → values stored in `inputStore`
3. User clicks Evaluate (or input changes with debounce) → `POST /evaluate-trace`
4. Response populates `traceStore` → dagre computes layout → nodes/edges render
5. Step-through: `stepStore` increments → nodes/edges transition through visual states
6. Play mode: auto-increment `stepStore` on a timer at configurable speed

## Scope Boundaries (v1)

**In scope:**
- Input Data nodes and Decision Table nodes only
- Full decision table rendering with matched-row highlighting
- Animated step-through and live re-evaluation
- Auto-generated input fields from TypeRegistry
- Dagre auto-layout

**Out of scope (future versions):**
- Business Knowledge Model (BKM) nodes
- Knowledge Source nodes
- Decision Service grouping
- Expression-level sub-stepping within a single cell
- Trace persistence / history
- Export (PDF, image)
- Multi-model comparison
