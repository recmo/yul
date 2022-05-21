use crate::Token;
use logos::{self, Logos};

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();
        Some((kind, text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_example() {
        let example = include_str!("../test/example.yul");
        let tokens = Lexer::new(example)
            .map(|(a, b)| (a, b.to_string()))
            .collect::<Vec<_>>();
        let expected: Vec<(Token, String)> =
            serde_json::from_str(include_str!("../test/example.lexer.json")).unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_erc20() {
        let example = include_str!("../test/erc20.yul");
        let tokens = Lexer::new(example)
            .map(|(a, b)| (a, b.to_string()))
            .collect::<Vec<_>>();
        let expected: Vec<(Token, String)> =
            serde_json::from_str(include_str!("../test/erc20.lexer.json")).unwrap();
        assert_eq!(tokens, expected);
    }
}
