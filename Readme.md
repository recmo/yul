# Yul language tools

[![crates.io](https://buildstats.info/crate/yul)](https://crates.io/crates/yul)
[![docs.rs](https://img.shields.io/docsrs/yul)](https://docs.rs/yul)
[![MIT License](https://img.shields.io/github/license/recmo/yul)](https://github.com/recmo/yul/blob/main/mit-license.md)
[![dependency status](https://deps.rs/repo/github/recmo/yul/status.svg)](https://deps.rs/repo/github/recmo/yul)
[![codecov](https://codecov.io/gh/recmo/yul/branch/main/graph/badge.svg?token=WBPZ9U4TTO)](https://codecov.io/gh/recmo/yul)
[![CI](https://github.com/recmo/yul/actions/workflows/ci.yml/badge.svg)](https://github.com/recmo/yul/actions/workflows/ci.yml)


## Parsing

The parser uses [Logos](https://docs.rs/logos/latest/logos) for tokenization and [Rowan](https://docs.rs/rowan/latest/rowan) for a parsing. The tokens (aka lexer) are defined in the [token.rs](./parser/src/token.rs) and the syntax (aka grammar) is defined in [syntax.rs](./parser/src/syntax.rs).

The architecture is based on that of [rust-analyzer]. In fact, `rowan` is part of the rust-analyzer project. I recommend reading their [write-up][ra-writeup] to understand `rowan`. Another great introduction is [this tutorial][rowan-tut] by Luna Razzaghipour.

Specifically the architecture is for *lossless parsing* with good error recovery. This means that
the original source file can always be perfectly reconstructed from the parse tree, even if it has errors. Comments, whitespace and parse errors are explicitly included in the parse tree.

[yul-spec]: https://docs.soliditylang.org/en/v0.8.14/yul.html#specification-of-yul
[rust-analyzer]: https://rust-analyzer.github.io/
[ra-writeup]: https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/syntax.md
[rowan-tut]: https://arzg.github.io/lang/10/


## Building and testing

Format, lint, build and test everything (I recommend creating a shell alias for this):

```sh
cargo fmt &&\
cargo clippy --all-features --all-targets &&\
cargo test --workspace --all-features --doc -- --nocapture &&\
cargo test --workspace --all-features --all-targets -- --nocapture &&\
cargo doc --workspace --all-features --no-deps
```

Run benchmarks with the provided `.cargo/config.toml` alias

```sh
cargo criterion
```

Check documentation coverage

```sh
RUSTDOCFLAGS="-Z unstable-options --show-coverage"  cargo doc --workspace --all-features --no-deps
```

## To do

Goals:

* Yul to EVM compiler
* Yul optimizer
* Yul language server

Maybe:

* EVM to yul compiler
* Yul LLVM compiler
* Static analysis tools like SMT checker
* Link in `solc` frontend for Solidity compatibility.

---

[![lines of code](https://img.shields.io/tokei/lines/github/recmo/yul)](https://github.com/recmo/yul)
[![GitHub contributors](https://img.shields.io/github/contributors/recmo/yul)](https://github.com/recmo/yul/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/recmo/yul)](https://github.com/recmo/yul/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/recmo/yul?label=PRs)](https://github.com/recmo/yul/pulls)
[![GitHub Repo stars](https://img.shields.io/github/stars/recmo/yul)](https://star-history.com/#recmo/yul&Date)
[![crates.io](https://img.shields.io/crates/d/yul)](https://crates.io/crates/yul)
