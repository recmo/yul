use crate::{Node, Parser, Token};

pub fn parse_file(p: &mut Parser) {
    p.start_node(Node::File);
    p.skip_insignificant();
    while p.peek().is_some() {
        parse_object(p);
    }
    p.finish_node();
}

pub fn parse_object(p: &mut Parser) {
    p.start_node(Node::Object);
    p.token(Token::Object);
    p.token(Token::LiteralString);
    p.token(Token::BraceOpen);
    // Parse `code { ... }`
    p.start_node(Node::Code);
    p.token(Token::Code);
    parse_block(p);
    p.finish_node();
    // Parse objects and raw data
    while p.peek_is_not(Token::BraceClose) {
        parse_data(p);
    }
    p.token(Token::BraceClose);
    p.finish_node();
}

pub fn parse_block(p: &mut Parser) {
    p.start_node(Node::Block);
    p.token(Token::BraceOpen);
    while p.peek_is_not(Token::BraceClose) {
        parse_statement(p);
    }
    p.token(Token::BraceClose);
    p.finish_node();
}

pub fn parse_data(p: &mut Parser) {
    use Token::*;
    if let Some(token) = p.peek() {
        match token {
            Object => parse_object(p),
            Data => {
                p.start_node(Node::Data);
                p.token(Data);
                p.token(LiteralString);
                if let Some(token) = p.peek() {
                    match token {
                        LiteralString => p.token(LiteralString),
                        LiteralStringHex => p.token(LiteralStringHex),
                        _ => p.error(),
                    }
                }
                p.finish_node();
            }
            _ => p.error(),
        }
    }
}

pub fn parse_statement(p: &mut Parser) {
    use Token::*;
    if let Some(token) = p.peek() {
        p.start_node(Node::Statement);
        match token {
            BraceOpen => parse_block(p),
            Function => {
                p.start_node(Node::Function);
                p.token(Function);
                p.token(Identifier);
                p.start_node(Node::Arguments);
                p.token(ParenOpen);
                if p.peek_is(Identifier) {
                    parse_indentifiers(p);
                }
                p.token(ParenClose);
                p.finish_node();
                p.start_node(Node::Returns);
                if p.peek_is(Returns) {
                    p.token(Returns);
                    parse_indentifiers(p);
                }
                p.finish_node();
                parse_block(p);
                p.finish_node();
            }
            Let => {
                p.start_node(Node::Let);
                p.token(Let);
                parse_indentifiers(p);
                if p.peek_is(Assign) {
                    p.token(Assign);
                    parse_expression(p);
                }
                p.finish_node();
            }
            If => {
                p.start_node(Node::If);
                p.token(If);
                parse_expression(p);
                parse_block(p);
                p.finish_node();
            }
            Switch => {
                p.start_node(Node::Switch);
                p.token(Switch);
                parse_expression(p);
                while p.peek_is(Case) {
                    p.start_node(Node::Case);
                    p.token(Case);
                    parse_expression(p); // TODO: Only accept literal.
                    parse_block(p);
                    p.finish_node();
                }
                if p.peek_is(Default) {
                    p.start_node(Node::CaseDefault);
                    p.token(Default);
                    parse_block(p);
                    p.finish_node();
                }
                p.finish_node();
            }
            For => {
                p.start_node(Node::For);
                p.token(For);
                parse_block(p);
                parse_expression(p);
                parse_block(p);
                parse_block(p);
                p.finish_node();
            }
            Break => {
                p.start_node(Node::Break);
                p.token(Break);
                p.finish_node();
            }
            Continue => {
                p.start_node(Node::Continue);
                p.token(Continue);
                p.finish_node();
            }
            Leave => {
                p.start_node(Node::Leave);
                p.token(Leave);
                p.finish_node();
            }
            _ => {
                // Here the grammar is a bit challenging, we can either have an
                // expression or an assignment. Both can start with an identifier.
                // If after the first identifier we get a Comma or Assign, we
                // know it will be an assignment. Otherwise it will be an expression.
                if p.peek_is(Identifier) {
                    let start = p.checkpoint();
                    p.token(Identifier);
                    match p.peek() {
                        Some(Comma) => {
                            p.start_node_at(start, Node::Assignment);
                            p.start_node_at(start, Node::Identifiers);
                            while p.peek_is(Comma) {
                                p.token(Comma);
                                p.token(Identifier);
                            }
                            p.finish_node();
                            p.token(Assign);
                            parse_expression(p);
                            p.finish_node();
                        }
                        Some(Assign) => {
                            p.start_node_at(start, Node::Assignment);
                            p.start_node_at(start, Node::Identifiers);
                            p.finish_node();
                            p.token(Assign);
                            parse_expression(p);
                            p.finish_node();
                        }
                        Some(ParenOpen) => {
                            // Call Expression
                            p.start_node_at(start, Node::Expression);
                            p.start_node_at(start, Node::Call);

                            p.start_node(Node::Arguments);
                            p.token(ParenOpen);
                            while p.peek_is_not(ParenClose) {
                                parse_expression(p);
                                if p.peek_is_not(ParenClose) {
                                    p.token(Comma);
                                }
                            }
                            p.token(ParenClose);
                            p.finish_node();
                            p.finish_node();

                            p.finish_node();
                        }
                        _ => {
                            // Identifier Expression
                            p.start_node_at(start, Node::Expression);
                            p.finish_node();
                        }
                    }
                } else {
                    parse_expression(p);
                }
            }
        }
        p.finish_node();
    }
}

pub fn parse_expression(p: &mut Parser) {
    use Token::*;
    if let Some(token) = p.peek() {
        p.start_node(Node::Expression);
        match token {
            Identifier => {
                let start = p.checkpoint();
                p.token(Identifier);
                if p.peek_is(ParenOpen) {
                    p.start_node_at(start, Node::Call);
                    p.start_node(Node::Arguments);
                    p.token(ParenOpen);
                    while p.peek_is_not(ParenClose) {
                        parse_expression(p);
                        if p.peek_is_not(ParenClose) {
                            p.token(Comma);
                        }
                    }
                    p.token(ParenClose);
                    p.finish_node();
                    p.finish_node();
                }
            }
            LiteralTrue | LiteralFalse | LiteralHex | LiteralInt | LiteralString => {
                p.start_node(Node::Literal);
                p.bump();
                p.skip_insignificant();
                p.finish_node();
            }
            _ => p.error(),
        }
        p.finish_node();
    }
}

pub fn parse_indentifiers(p: &mut Parser) {
    use Token::*;
    p.start_node(Node::Identifiers);
    p.token(Identifier);
    while p.peek_is(Comma) {
        p.token(Comma);
        p.token(Identifier);
    }
    p.finish_node();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let input = include_str!("../test/example.yul");
        let mut parser = Parser::new(input);
        parse_file(&mut parser);
        let parse = parser.finish();
        assert_eq!(
            format!("{:#?}", &parse),
            include_str!("../test/example.parse.txt")
        );
        let unparse = parse.unparse();
        assert_eq!(input, unparse);
    }

    #[test]
    fn parse_erc20() {
        let input = include_str!("../test/erc20.yul");
        let mut parser = Parser::new(input);
        parse_file(&mut parser);
        let parse = parser.finish();
        assert_eq!(
            format!("{:#?}", &parse),
            include_str!("../test/erc20.parse.txt")
        );
        let unparse = parse.unparse();
        assert_eq!(input, unparse);
    }
}
