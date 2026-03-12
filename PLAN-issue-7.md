# Implementation Plan: Type System for Markdown-Native DMN (Issue #7)

## TDD Approach

Each task follows **Red-Green-Refactor**: write failing tests first, implement the minimum code to pass, then refactor.

---

## Phase 1 — Type Registry

### 1.1 Create `dsntk-type-registry` crate scaffold
- Add new crate `type-registry/` to the workspace
- Define core data structures: `TypeRegistry`, `TypeEntry`, `TypeSource`
- Define error types using `dsntk-common` patterns (`#[derive(ToErrorMessage)]`)
- **Tests (Red):**
  - `test_empty_registry_has_no_types()`
  - `test_registry_insert_and_lookup()`
  - `test_registry_returns_none_for_unknown_type()`

### 1.2 Primitive type resolution (no schema file needed)
- Implement resolution of FEEL primitives (`number`, `string`, `boolean`, `date`) from `data-type.ref` without requiring `schema`
- Map primitives to existing `FeelType` variants
- **Tests (Red → Green):**
  - `test_primitive_type_ref_resolves_without_schema()` — `number` → `FeelType::Number`
  - `test_all_primitive_types_resolve()` — `string`, `boolean`, `date`, etc.
  - `test_unknown_primitive_returns_error()`

### 1.3 TypeScript file parsing with `oxc_parser`
- Add `oxc_parser`, `oxc_ast`, `oxc_allocator`, `oxc_span` to workspace dependencies
- Implement `ts_parser` module: parse `.ts` files, extract exported `interface` and `type` declarations
- Map TS types to `FeelType`: `number`→Number, `string`→String, `boolean`→Boolean, union literals→enum/allowed-values, nested objects→Context
- **Tests (Red → Green):**
  - `test_typescript_file_ref_resolves_named_type()` — parse an interface and find it by name
  - `test_typescript_extracts_interface_fields()` — field names and types are correct
  - `test_typescript_union_type_extracts_enum_values()` — `"a" | "b"` → allowed values
  - `test_typescript_optional_field_marked_nullable()` — `age?: number` → optional
  - `test_typescript_nested_object_becomes_context()` — inline `{ street: string }` → FeelType::Context
  - `test_typescript_non_exported_types_ignored()` — only `export` declarations are registered

### 1.4 JSON Schema file parsing
- Add `serde_json` (already in workspace) for JSON Schema reading
- Implement `json_schema_parser` module: extract type definitions by `$id` or `title`
- Map JSON Schema types to `FeelType`: `"type": "number"` → Number, `"type": "object"` → Context, `"enum"` → allowed values
- **Tests (Red → Green):**
  - `test_json_schema_resolves_by_id()` — `$id` field used as registry key
  - `test_json_schema_resolves_by_title_fallback()` — falls back to `title` if no `$id`
  - `test_json_schema_object_maps_to_context_type()`
  - `test_json_schema_enum_extracts_allowed_values()`
  - `test_json_schema_required_vs_optional_fields()`

### 1.5 Folder scanning and ambiguity detection
- Implement folder scanner: given a directory path, scan all `.ts` and `.json` files, build a unified `TypeRegistry`
- Detect and error on ambiguous type names (same name in multiple files)
- **Tests (Red → Green):**
  - `test_folder_ref_resolves_type_across_multiple_files()` — type from `a.ts` found when scanning folder
  - `test_folder_ref_missing_type_name_errors_clearly()` — descriptive error if type not found
  - `test_ambiguous_type_name_across_folder_errors_with_both_paths()` — error names both conflicting files
  - `test_folder_scan_ignores_non_schema_files()` — `.rs`, `.md` files skipped

### 1.6 Front-matter parsing for `data-type` block
- Extend markdown parsing to handle YAML front matter with `data-type.ref` and `data-type.schema`
- Parse the three forms: primitive-only, file reference, folder reference
- **Tests (Red → Green):**
  - `test_frontmatter_parses_primitive_data_type()`
  - `test_frontmatter_parses_file_schema_ref()`
  - `test_frontmatter_parses_folder_schema_ref()`
  - `test_frontmatter_missing_ref_errors()`

### 1.7 `dsntk types` CLI command
- Add `types` subcommand to the CLI in `dsntk/`
- List all resolved types with source file information
- Add `--check` flag for validation-only mode (no output, exit code only)
- **Tests (Red → Green):**
  - `test_cli_types_lists_resolved_types()` — integration test with sample directory
  - `test_cli_types_check_succeeds_on_valid_project()`
  - `test_cli_types_check_fails_on_missing_type()`

---

## Phase 2 — Validation

### 2.1 Enum/union constraint checking
- Validate that decision table cell values conform to enum/union type constraints
- Compare cell literal values against `allowed_values` from resolved types
- **Tests (Red → Green):**
  - `test_cell_value_matches_typescript_enum_passes_validation()`
  - `test_cell_value_violates_typescript_enum_fails_validation()`
  - `test_cell_value_against_json_schema_enum()`

### 2.2 Optional field / null handling
- TypeScript `field?: type` and JSON Schema non-required fields allow null/absent input values
- **Tests (Red → Green):**
  - `test_optional_typescript_field_allows_null_input()`
  - `test_required_field_rejects_null_input()`
  - `test_json_schema_optional_field_allows_null()`

### 2.3 Nested context type resolution
- Validate field chains like `Applicant.address.postcode` through nested Context types
- **Tests (Red → Green):**
  - `test_nested_context_type_resolves_field_chain()`
  - `test_nested_context_missing_field_errors()`
  - `test_deeply_nested_context_resolves()`

### 2.4 Input payload validation via `jsonschema` crate
- Add `jsonschema` crate dependency
- Validate runtime input data against resolved JSON Schema
- **Tests (Red → Green):**
  - `test_valid_input_passes_json_schema_validation()`
  - `test_invalid_input_fails_json_schema_validation()`
  - `test_input_data_missing_type_triggers_warning()`

### 2.5 Type compatibility across information requirement edges
- When Decision A requires Input Data B, verify B's resolved type is compatible with A's expected input type
- **Tests (Red → Green):**
  - `test_compatible_types_across_edge_passes()`
  - `test_incompatible_types_across_edge_fails()`

---

## Phase 3 — BKM & Knowledge Source Semantics

### 3.1 BKM signature parsing
- Parse `signature` block from BKM front matter: `parameters` (name + type) and `return-type`
- Map to `FeelType::Function(params, return_type)`
- **Tests (Red → Green):**
  - `test_bkm_signature_parses_parameter_types()`
  - `test_bkm_missing_return_type_fails_validation()`
  - `test_bkm_complex_parameter_types_resolve()`

### 3.2 FEEL body inspection for BKM invocations
- Scan FEEL expressions in BKM bodies for function call names
- Cross-reference call names against known BKM IDs
- **Tests (Red → Green):**
  - `test_bkm_feel_expression_body_parsed()`
  - `test_bkm_invocation_resolves_to_known_node()`
  - `test_bkm_invocation_unknown_function_errors()`

### 3.3 BKM invocation arity and type validation
- Verify function call arguments match declared parameter count and types
- **Tests (Red → Green):**
  - `test_bkm_invocation_arity_mismatch_is_rejected()`
  - `test_bkm_invocation_type_mismatch_is_rejected()`
  - `test_bkm_invocation_correct_call_passes()`

### 3.4 Knowledge Source node support
- Parse `type: knowledge-source` front matter with `uri` and `owner` fields
- Knowledge Sources have no body — they are authority metadata only
- **Tests (Red → Green):**
  - `test_knowledge_source_without_body_is_valid()`
  - `test_knowledge_source_uri_preserved_in_graph()`

### 3.5 Authority requirement edge types
- Implement `governed-by` (authority requirements → Knowledge Sources) and `supported-by` (knowledge requirements → BKMs) link keys
- Enforce that `governed-by` targets `type: knowledge-source` nodes and `supported-by` targets `type: bkm` nodes
- Distinguish these semantically from `requires` (information requirements)
- **Tests (Red → Green):**
  - `test_governed_by_resolves_to_knowledge_source_node()`
  - `test_governed_by_non_knowledge_source_is_invalid()`
  - `test_supported_by_resolves_to_bkm_node()`
  - `test_edge_types_distinguished_in_graph()`

---

## Phase 4 — Export

### 4.1 TypeScript types → DMN `ItemDefinition` XML
- Convert resolved TypeScript types in the registry to DMN `ItemDefinition` XML elements
- Handle: simple types, enums (UnaryTests), contexts (item components), collections
- **Tests (Red → Green):**
  - `test_simple_type_exports_to_item_definition_xml()`
  - `test_enum_type_exports_with_allowed_values()`
  - `test_context_type_exports_with_item_components()`
  - `test_nested_type_exports_recursively()`

### 4.2 JSON Schema → DMN `ItemDefinition` XML
- Convert resolved JSON Schema types to DMN `ItemDefinition` XML
- **Tests (Red → Green):**
  - `test_json_schema_exports_to_item_definition_xml()`
  - `test_json_schema_enum_exports_allowed_values()`
  - `test_json_schema_nested_object_exports()`

### 4.3 Full model DMN XML export with type definitions
- Integrate type export into the complete model XML generation pipeline
- Ensure `ItemDefinition` elements are placed correctly in the DMN `Definitions` root
- **Tests (Red → Green):**
  - `test_full_model_export_includes_item_definitions()`
  - `test_exported_xml_validates_against_dmn_xsd()`

---

## Cross-Cutting Concerns

### Error messages
- All errors include the component name prefix (e.g., `[type-registry]`)
- File-related errors include the file path and line number where applicable
- Ambiguity errors name both conflicting files

### Documentation
- Update CLI help text for `dsntk types` command
- Add example type files in `examples/` crate

### Integration
- Ensure the new crate fits the workspace dependency flow:
  ```
  dsntk-type-registry
    ├── dsntk-feel (for FeelType)
    ├── dsntk-common (for errors)
    └── oxc_parser, serde_json, jsonschema (external)
  ```
- Wire into `dsntk-model-evaluator` for type resolution during evaluation
- Wire into `dsntk` CLI for the `types` subcommand
