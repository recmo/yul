# Yul parser

Parses the Yul programming language. See the [yul specification][yul-spec].

The parser uses [Logos][https://docs.rs/logos/latest/logos] for tokenization and [Rowan][https://docs.rs/rowan/latest/rowan] for a parsing. The tokens (aka lexer) are defined in the [token.rs][./src/tokens.rs] and the syntax (aka grammar) is defined in [syntax.rs][./src/syntax.rs].

The architecture is based on that of [rust-analyzer]. In fact, `rowan` is part of the rust-analyzer project. I recommend reading their [write-up][ra-writeup] to understand `rowan`. Another great introduction is [this tutorial][rowan-tut] by Luna Razzaghipour.

Specifically the architecture is for *lossless parsing* with good error recovery. This means that
the original source file can always be perfectly reconstructed from the parse tree, even if it has errors. Comments, whitespace and parse errors are explicitly included in the parse tree.

[yul-spec]: https://docs.soliditylang.org/en/v0.8.14/yul.html#specification-of-yul
[rust-analyzer]: https://rust-analyzer.github.io/
[ra-writeup]: https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/syntax.md
[rowan-tut]: https://arzg.github.io/lang/10/
