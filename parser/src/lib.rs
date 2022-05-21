//! Yul parser
//!
//! See <https://solidity.readthedocs.io/en/v0.7.3/yul.html#specification-of-yul>

// Following <https://arzg.github.io/lang/10/>
// <https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/syntax.md>

// pub mod ast;
// mod ir;
mod lexer;
mod node;
mod parse;
mod parser;
mod syntax;
mod syntax_kind;
mod token;

// pub use parser::{parse_block, parse_file, parse_object};
pub use lexer::Lexer;
pub use node::Node;
pub use parse::Parse;
pub use parser::Parser;
pub use syntax_kind::{SyntaxKind, SyntaxNode, YulLanguage};
pub use token::Token;

pub fn parse(input: &str) -> Parse {
    let mut parser = Parser::new(input);
    syntax::parse_file(&mut parser);
    parser.finish()
}
