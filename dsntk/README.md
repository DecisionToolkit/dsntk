# DecisionToolkit

[![Crates.io][crates-badge]][crates-url]
[![docs][docs-badge]][docs-url]
![Code coverage][coverage-badge]<br/>
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]<br/>
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]

[crates-badge]: https://img.shields.io/crates/v/dsntk.svg
[crates-url]: https://crates.io/crates/dsntk
[docs-badge]: https://img.shields.io/badge/docs-passing-green.svg
[docs-url]: https://dsntk.io
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-97%25-green.svg
[build-badge-linux]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/DecisionToolkit/dsntk/actions/workflows/build-macos.yml/badge.svg
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/DecisionToolkit/dsntk/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/DecisionToolkit/dsntk/blob/main/LICENSE
[apache-notice-url]: https://github.com/DecisionToolkit/dsntk/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/DecisionToolkit/dsntk/blob/main/CODE_OF_CONDUCT.md
[github-url]: https://github.com/DecisionToolkit

### [Installation](https://decision-toolkit.org/guide/installation.html) | [Documentation](https://decision-toolkit.org)

## Overview

**DecisionToolkit** is a set of tools designed to build, test and evaluate decision models,
constructed basing on the [Decision Model and Notation](https://www.omg.org/dmn) (DMN™)
specification, which is an industry standard governed by the [Object Management Group](https://www.omg.org) (OMG®).

**DecisionToolkit** aims to be performant, reliable, and fully compliant with the DMN™ specification,
ensuring accurate evaluation of decision models. All tools are implemented in [Rust](https://www.rust-lang.org/),
a programming language known for its security, efficiency and reliability.

**DecisionToolkit** offers a range of features, including DMN™ models evaluation, decision tables evaluation,
and FEEL expressions evaluation. It also includes functionality for parsing DMN™ models, decision tables,
and FEEL expressions. Users can test DMN™ models, decision tables, and FEEL expressions,
and export them to HTML.

### Features:
- [Serving DMN™ models](https://decision-toolkit.org/guide/commands/command-srv.html)
- [Evaluating DMN™ models](https://decision-toolkit.org/guide/commands/command-edm.html)
- [Evaluating decision tables](https://decision-toolkit.org/guide/commands/command-edt.html)
- [Evaluating FEEL expressions](https://decision-toolkit.org/guide/commands/command-efe.html)
- [Parsing DMN™ models](https://decision-toolkit.org/guide/commands/command-pdm.html)
- [Parsing decision tables](https://decision-toolkit.org/guide/commands/command-pdt.html)
- [Parsing FEEL expressions](https://decision-toolkit.org/guide/commands/command-pfe.html)
- [Testing DMN™ models](https://decision-toolkit.org/guide/commands/command-tdm.html)
- [Testing decision tables](https://decision-toolkit.org/guide/commands/command-tdt.html)
- [Testing FEEL expressions](https://decision-toolkit.org/guide/commands/command-tfe.html)
- [Exporting DMN™ models](https://decision-toolkit.org/guide/commands/command-xdm.html)
- [Exporting decision tables](https://decision-toolkit.org/guide/commands/command-xdt.html)
- [Exporting FEEL expressions](https://decision-toolkit.org/guide/commands/command-xfe.html)
- [Saving examples](https://decision-toolkit.org/guide/commands/command-exs.html)

## Status

**DecisionToolkit** is **PRODUCTION READY**, although some features
may still be refined or changed based on testing and user feedback.
We encourage users to try **DecisionToolkit** and share their feedback
to help us enhance its usability and performance.

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [**DecisionToolkit**][github-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
