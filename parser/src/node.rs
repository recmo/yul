use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive, Serialize, Deserialize)]
pub enum Node {
    Root,
    File,
    Object,
    Code,
    Block,
    Statement,
    Function,
    Arguments,
    Returns,
    If,
    Switch,
    Case,
    CaseDefault,
    Let,
    Assignment,
    For,
    Continue,
    Break,
    Leave,
    Expression,
    Call,
    Literal,
    Identifiers,
    Data,
    Error,
}

impl Node {
    pub fn root() -> Self {
        Node::Root
    }

    pub fn error() -> Self {
        Node::Error
    }
}
