# Installation

#DSNTK executable is a standalone file without additional dependencies and can be obtained by
downloading from [GitHub releases](https://github.com/dsntk/dsntk/releases),
or built from source using [Rust and Cargo](https://www.rust-lang.org/tools/install).

## Download binary

- Head to [GitHub releases](https://github.com/dsntk/dsntk/releases).
- Expand the **Assets** section in the newly released version.
- Download the binary of your choice, according to the operating system you use.

::: tip HINT

The names of released binary files contain the version number and operating system name.
In order to proceed with the examples presented in this documentation,
rename the downloaded binaries as shown below.

::: code-group

```shell [Linux (x86_64)]
$ mv dsntk-#VERSION-linux-x86_64 dsntk
$ chmod +x dsntk
```

```shell [Windows (x86_64)]
> rename dsntk-#VERSION-windows-x86_64.exe dsntk.exe
```

```shell [macOs (x86_64)]
$ mv dsntk-#VERSION-apple-macos-x86_64 dsntk
$ chmod +x dsntk
```

```shell [macOs (ARM64)]
$ mv dsntk-#VERSION-apple-macos-arm64 dsntk
$ chmod +x dsntk
```

:::

## Build from source

Install [Rust and Cargo](https://www.rust-lang.org/tools/install) and type:

::: code-group

```shell [TERMINAL]
cargo install dsntk
```

:::
