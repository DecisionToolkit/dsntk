[dsntk] | [ÐecisionToolkit]

# Black-box tests

[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/DecisionToolkit/dsntk/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/DecisionToolkit/dsntk/blob/main/LICENSE
[apache-notice-url]: https://github.com/DecisionToolkit/dsntk/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/DecisionToolkit/dsntk/blob/main/CODE_OF_CONDUCT.md
[ÐecisionToolkit]: https://github.com/DecisionToolkit
[ÐecisionToolkit]: https://github.com/DecisionToolkit
[dsntk]: https://crates.io/crates/dsntk

## Overview

Black-box tests for **[DecisionToolkit]** project.

The **DecisionToolkit** project is tested automatically with all types of tests provided by Rust,
and additionally with test runner that executes all available compatibility tests.

Nevertheless, this set of black-box tests is maintained in the following purposes:

- testing the edge cases,
- diagnostics of reported issues,
- visualization of functionalities provided by **DecisionToolkit**,
- preparing documentation ([decision-toolkit.org][ÐecisionToolkit]).

## Running black-box tests

Run all tests:

```
$ ./bbt.sh
```

Run tests in specified directory, e.g. in `cli/noargs`:

```
$ ./bbt.sh cli/noargs
```

## Test directories structure

Tests are organized in directories starting from the root directory named `./tests`.
Each directory may contain either subdirectories or test files.

## Test files structure

There are always four test files prepared for each test:

1. Text file containing tested expression, decision table or DMN model, may have any name.
2. Text file containing test execution context, may have any name.
3. Text file containing expected result, should always be named `expected`.
4. Shell script containing a command that runs a test, should always be named `run.sh`.

## Example test

Directory `tests/feel/addition/0001` contains a test that checks addition operation of two numbers:

1. Tested expression is `1 + 1` and is saved in `0001.feel` file.
2. Tested expression context is empty `{}` and is saved in `0001.ctx` file.
3. Expected value is `2` and is saved in `expected` file.
4. Script that executes a test contains a command `dsntk efe 0001.ctx 0001.feel` and is saved in `run.sh` file.

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to **[ÐecisionToolkit]** are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
