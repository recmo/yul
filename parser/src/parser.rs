use crate::{Lexer, Node, Parse, SyntaxKind, Token, YulLanguage};
use core::iter::Peekable;
use rowan::{Checkpoint, GreenNodeBuilder, Language};

/// Wrapper around Logos and Rowan to for parsing with 1 lookahead.
pub struct Parser<'a> {
    lexer:   Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Self {
            lexer:   Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        };
        parser.start_node(Node::root());
        parser
    }

    /// Finishes the parse, any remaining content will be consumed as an error.
    pub fn finish(mut self) -> Parse {
        if self.peek().is_some() {
            self.start_node(Node::Error);
            while self.peek().is_some() {
                self.bump();
            }
            self.finish_node();
        }
        self.finish_node();
        Parse::new(self.builder.finish())
    }

    /// True if the next token is `token`.
    pub fn peek_is(&mut self, token: Token) -> bool {
        self.peek().map_or(false, |peek| peek == token)
    }

    /// True if the next token is not `token` or end-of-file.
    pub fn peek_is_not(&mut self, token: Token) -> bool {
        self.peek().map_or(false, |peek| peek != token)
    }

    /// Peek at the first unprocessed token
    pub fn peek(&mut self) -> Option<Token> {
        self.lexer.peek().map(|(token, _)| *token)
    }

    /// Parse a specific token and skip trailing insignificants
    pub fn token(&mut self, token: Token) {
        if self.peek() == Some(token) {
            self.bump();
        } else {
            self.start_node(Node::Error);
            if self.peek().is_some() {
                self.bump();
            }
            self.finish_node();
            // todo!();
        }
        self.skip_insignificant();
    }

    /// Add the next significant token in an error node.
    pub fn error(&mut self) {
        self.start_node(Node::error());
        self.bump();
        self.skip_insignificant();
        self.finish_node();
    }

    /// Skip insignificant tokens
    pub fn skip_insignificant(&mut self) {
        while !self.peek().as_ref().map_or(true, Token::is_significant) {
            self.bump()
        }
    }

    /// Advance one token, adding it to the current branch of the tree builder.
    ///
    /// # Panics
    /// Panics when called on end-of-file.
    pub fn bump(&mut self) {
        let (token, text) = self.lexer.next().unwrap();
        self.add_token(token, text);
    }

    /// Prepare for maybe wrapping the next node.
    pub fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    /// Wrap the previous branch marked by `checkpoint` in a new branch and make
    /// it current.
    pub fn start_node_at(&mut self, checkpoint: Checkpoint, node: Node) {
        let kind = SyntaxKind::Node(node);
        self.builder
            .start_node_at(checkpoint, YulLanguage::kind_to_raw(kind));
    }

    /// Add a new parent node to the tree and decent.
    pub fn start_node(&mut self, node: Node) {
        let kind = SyntaxKind::Node(node);
        self.builder.start_node(YulLanguage::kind_to_raw(kind));
    }

    /// Finish the current branch and ascent to the parent node.
    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    /// Add a token to the current branch of the tree builder.
    pub fn add_token(&mut self, token: Token, slice: &str) {
        let kind = SyntaxKind::Token(token);
        self.builder
            .token(YulLanguage::kind_to_raw(kind), slice.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_nothing() {
        let parser = Parser::new("");
        let parse = parser.finish();
        assert_eq!(parse.debug_tree(), "Root@0..0");
    }

    #[test]
    fn parse_number() {
        let mut parser = Parser::new("123");
        parser.token(Token::LiteralInt);
        let parse = parser.finish();
        assert_eq!(
            serde_json::to_value(&parse).unwrap(),
            json!({
                "kind": "Root",
                "text_range": [0, 3],
                "children": [{
                    "kind": "LiteralInt",
                    "text_range": [0, 3],
                    "text": "123"
                }]
            })
        );
    }
}
