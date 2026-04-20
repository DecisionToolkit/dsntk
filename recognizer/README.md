[dsntk] | [ÐecisionToolkit]

### dsntk-recognizer

[![crates.io][crates-badge]][crates-url]
[![docs][docs-badge]][docs-url]
[![coverage][cov-badge]][cov-url]
![build-linux-gnu][build-badge-linux-gnu]
![build-linux-musl][build-badge-linux-musl]
![build-macos][build-badge-macos]
![build-macos-aarch64][build-badge-macos-aarch64]
![build-windows][build-badge-windows]
[![mit-license][mit-badge]][mit-license-url]
[![apache-license][apache-badge]][apache-license-url]
[![cc][cc-badge]][cc-url]

[crates-badge]: https://img.shields.io/crates/v/dsntk-recognizer.svg
[crates-url]: https://crates.io/crates/dsntk-recognizer
[docs-badge]: https://docs.rs/dsntk-recognizer/badge.svg
[docs-url]: https://crates.io/crates/dsntk-recognizer
[cov-badge]: https://img.shields.io/badge/coverage-0%25-21b577.svg
[cov-url]: https://crates.io/crates/coverio
[build-badge-linux-gnu]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-linux-gnu.yml/badge.svg
[build-badge-linux-musl]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-linux-musl.yml/badge.svg
[build-badge-macos]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-aarch64]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-macos-aarch64.yml/badge.svg
[build-badge-windows]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-windows.yml/badge.svg
[mit-badge]: https://img.shields.io/badge/License-MIT-9370DB.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/DecisionToolkit/dsntk/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-9370DB.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/DecisionToolkit/dsntk/blob/main/LICENSE
[apache-notice-url]: https://github.com/DecisionToolkit/dsntk/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-9370DB.svg
[cc-url]: https://github.com/DecisionToolkit/dsntk/blob/main/CODE_OF_CONDUCT.md
[repository-url]: https://github.com/DecisionToolkit/dsntk
[ÐecisionToolkit]: https://github.com/DecisionToolkit
[dsntk]: https://crates.io/crates/dsntk

# Decision table recognizer

## Overview

Decision table recognizer used by [**dsntk**](https://crates.io/crates/dsntk) crate.

Recognizes decision tables defined as Unicode or Markdown text.
To be properly recognized, the structure of the decision table must be conformant to DMN™ standard.

Example of decision table defined using Unicode characters:

```text
  ┌───┬────────────┬───────╥──────────┐
  │ U │  Customer  │ Order ║ Discount │
  ╞═══╪════════════╪═══════╬══════════╡
  │ 1 │ "Business" │  <10  ║   0.10   │
  ├───┼────────────┼───────╫──────────┤
  │ 2 │ "Business" │ >=10  ║   0.15   │
  ├───┼────────────┼───────╫──────────┤
  │ 3 │ "Private"  │   -   ║   0.05   │
  └───┴────────────┴───────╨──────────┘
```

Example of decision table defined using Markdown:

| U |  Customer  | Order | Discount |
|:-:|:----------:|:-----:|:--------:|
|   |    `i`     |  `i`  |   `o`    |
| 1 | "Business" |  <10  |   0.10   |
| 2 | "Business" | >=10  |   0.15   |
| 3 | "Private"  |   -   |   0.05   |

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to **[ÐecisionToolkit]** are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
