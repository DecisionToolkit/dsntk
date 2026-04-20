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
