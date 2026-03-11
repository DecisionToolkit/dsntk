# CLAUDE.md

## Project Overview

**dsntk-rs** (DecisionToolkit) is a suite of tools for building, testing, and evaluating decision models based on the **Decision Model and Notation (DMN)** standard (OMG). Implemented entirely in Rust, it provides a FEEL expression evaluator, DMN model parser/evaluator, decision table recognizer, REST API server, and CLI.

- **Version:** 0.3.0-dev
- **License:** MIT OR Apache-2.0
- **Rust Edition:** 2021
- **Repository:** https://github.com/DecisionToolkit/dsntk-rs

## Quick Reference Commands

```bash
# Build
cargo +stable build --workspace          # Debug build
cargo +stable build --release            # Release build

# Test
cargo +stable test --workspace           # Run all tests
cargo +stable test -p dsntk-feel-parser  # Test a specific crate

# Lint & Format
cargo +nightly fmt --all                 # Format code
cargo +stable clippy --workspace         # Lint (stable)
cargo +nightly clippy --workspace --all-targets  # Lint (nightly)

# Black-box tests (requires prior build)
cargo +stable build --workspace && cd bbt && ./bbt.sh && cd ..
```

If [Task](https://taskfile.dev) is installed, equivalent shortcuts are available: `task build`, `task test`, `task fmt`, `task clippy`, `task bbt`.

## Workspace Structure

This is a Cargo workspace with 18 member crates:

| Crate | Directory | Purpose |
|---|---|---|
| `dsntk` | `dsntk/` | CLI entry point |
| `dsntk-common` | `common/` | Shared types, errors, utilities |
| `dsntk-macros` | `macros/` | Procedural and derive macros |
| `dsntk-feel` | `feel/` | FEEL type definitions |
| `dsntk-feel-number` | `feel-number/` | Decimal floating-point number type |
| `dsntk-feel-temporal` | `feel-temporal/` | Date, time, and duration types |
| `dsntk-feel-regex` | `feel-regex/` | FEEL regex support |
| `dsntk-feel-grammar` | `feel-grammar/` | FEEL grammar and parsing tables |
| `dsntk-feel-parser` | `feel-parser/` | FEEL expression parser |
| `dsntk-feel-evaluator` | `feel-evaluator/` | FEEL expression evaluator |
| `dsntk-model` | `model/` | DMN model definition and XML parser |
| `dsntk-model-evaluator` | `model-evaluator/` | DMN model evaluator |
| `dsntk-evaluator` | `evaluator/` | High-level evaluator facade |
| `dsntk-recognizer` | `recognizer/` | Decision table recognizer |
| `dsntk-server` | `server/` | REST API server (Actix-web) |
| `dsntk-gendoc` | `gendoc/` | Documentation generator |
| `dsntk-examples` | `examples/` | Built-in example models |
| `dsntk-workspace` | `workspace/` | User workspace utilities |

### Dependency Flow

```
dsntk (CLI)
  └── dsntk-server
        └── dsntk-evaluator (facade)
              ├── dsntk-model-evaluator
              │     ├── dsntk-model (XML parsing)
              │     ├── dsntk-feel-evaluator
              │     │     ├── dsntk-feel-parser
              │     │     │     └── dsntk-feel-grammar
              │     │     ├── dsntk-feel-number
              │     │     ├── dsntk-feel-temporal
              │     │     └── dsntk-feel-regex
              │     └── dsntk-recognizer
              └── dsntk-workspace
  └── dsntk-gendoc
  └── dsntk-examples
  └── dsntk-common (used by all)
  └── dsntk-macros (used by all)
```

## Code Conventions

### Formatting

Defined in `rustfmt.toml`:
- **Max line width:** 180 characters
- **Indentation:** 2 spaces
- **Import granularity:** Module-level

Always format with `cargo +nightly fmt --all` before committing.

### Module Structure Pattern

Each crate follows this pattern in `lib.rs`:

```rust
#[macro_use]
extern crate dsntk_macros;

mod implementation;
mod errors;

#[cfg(test)]
mod tests;

pub use implementation::PublicType;
pub use errors::Result;
```

### Naming Conventions

- **Crate names:** `dsntk-<component>` (kebab-case)
- **Import names:** `dsntk_<component>` (snake_case)
- **Test functions:** Numeric suffix pattern `_NNNN` (e.g., `fn _0001()`, `fn _0002()`)
- **Test input files:** Prefixed with `T_` or `t_` (e.g., `t_0001.dmn`)
- **Source files:** Lowercase with underscores

### Error Handling

All crates use a unified error type from `dsntk-common`:

```rust
// Base type
pub type Result<T, E = DsntkError> = std::result::Result<T, E>;

// DsntkError wraps a string with source prefix: "<source> message"
```

Each crate defines its own error factory functions in `errors.rs` using the `#[derive(ToErrorMessage)]` macro from `dsntk-macros`. Error messages always include the component name for traceability.

## Testing

### Test Types

1. **Unit tests** - Inline in source files or in `src/tests/` directories with `#[cfg(test)]`
2. **Integration tests** - In `tests/` directories at crate root
3. **Compatibility tests** - In `src/tests/compatibility/` for DMN standard compliance
4. **Black-box tests (BBT)** - In `bbt/` directory, each test has:
   - A FEEL/DMN input file (e.g., `0001.feel`)
   - A context file (e.g., `0001.ctx`)
   - An `expected` file with expected output
   - A `run.sh` script

### Running Specific Test Suites

```bash
cargo +stable test -p dsntk-feel-parser      # Single crate
cargo +stable test -p dsntk-model-evaluator   # Model evaluator
cd bbt && ./bbt.sh cli/noargs                 # BBT in specific directory
```

### Code Coverage

Coverage target is 95%+. Generate reports with:
```bash
./Coverage.sh                       # All crates
./Coverage-crate.sh dsntk-feel      # Specific crate
```

## Feature Flags

- **`tck`** - Enables Technology Compatibility Kit test mode
- **`parsing-tables`** - Enables parser table generation

```bash
cargo +stable build --workspace --features=tck
cargo +stable clippy --workspace --all-targets --features=tck
```

## CI/CD

GitHub Actions workflows in `.github/workflows/` build for four targets:
- `x86_64-unknown-linux-musl`
- `x86_64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

Triggered on pushes to `main` and `releases/**` branches.

## Key Technical Details

- **XML parsing:** Uses `roxmltree` for DMN model XML
- **HTTP server:** Actix-web 4.x
- **Number handling:** Custom decimal floating-point via `dfp-number-sys`
- **CLI:** Built with `clap` (cargo feature)
- **DMN schemas:** XSD schemas for versions 1.3, 1.4, and 1.5 in `schemas/`

## Things to Avoid

- Do not add dependencies without workspace-level declaration in root `Cargo.toml`
- Do not use `rustfmt` without `+nightly` (the config requires nightly features)
- Do not skip clippy checks; both stable and nightly clippy must pass
- Do not modify generated parsing tables in `feel-grammar` without the `parsing-tables` feature
- Do not break the numeric test naming convention (`_NNNN` pattern)
