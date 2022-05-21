use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, PartialEq, Logos, Debug, FromPrimitive, ToPrimitive, Serialize, Deserialize,
)]
pub enum Token {
    // Keywords
    #[token("object")]
    Object,
    #[token("code")]
    Code,
    #[token("data")]
    Data,
    #[token("function")]
    Function,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("for")]
    For,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("leave")]
    Leave,

    // Syntax
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token(r"{")]
    BraceOpen,
    #[token(r"}")]
    BraceClose,
    #[token(",")]
    Comma,
    #[token(":=")]
    Assign,
    #[token("->")]
    Returns,
    #[token(":")]
    Typed,

    // Identifiers
    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9.]*")]
    Identifier,

    // Literals
    #[token("true")]
    LiteralTrue,
    #[token("false")]
    LiteralFalse,
    #[regex(r"0x[0-9a-fA-F]+")]
    LiteralHex,
    #[regex(r"[0-9]+")]
    LiteralInt,

    #[regex(r#""([^"\r\n\\]|\\.)*""#)]
    LiteralString,
    #[regex(r#"hex"([0-9a-fA-F][0-9a-fA-F])*""#)]
    LiteralStringHex,

    // Ignored syntax
    #[regex(r#"//[^\n]*"#)]
    LineComment,
    // TODO: Allow * in block comment when not followed by /
    // See <https://stackoverflow.com/questions/16160190/regular-expression-to-find-c-style-block-comments>
    #[regex(r#"/\*[^*]*\*/"#)]
    BlockComment,
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}

impl Token {
    /// Significant tokens are not whitespace, comments, etc.
    pub fn is_significant(&self) -> bool {
        use Token::*;
        match self {
            LineComment | BlockComment | Whitespace => false,
            _ => true,
        }
    }

    /// Token represents a lexer error
    pub fn is_error(&self) -> bool {
        use Token::*;
        match self {
            Error => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_example() {
        let example = include_str!("../test/example.yul");
        let tokens = Token::lexer(example).collect::<Vec<_>>();
        let expected: Vec<Token> =
            serde_json::from_str(include_str!("../test/example.tokens.json")).unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_erc20() {
        let example = include_str!("../test/erc20.yul");
        let tokens = Token::lexer(example).collect::<Vec<_>>();
        let expected: Vec<Token> =
            serde_json::from_str(include_str!("../test/erc20.tokens.json")).unwrap();
        assert_eq!(tokens, expected);
    }
}
