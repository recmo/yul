#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

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
