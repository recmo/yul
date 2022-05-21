use serde::{Deserialize, Serialize};
use zkp_u256::U256;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SourceFile {
    pub objects: Vec<Object>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Object {
    pub name: String,
    pub code: Vec<Statement>,
    pub data: Vec<ObjectData>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ObjectData {
    Object(Object),
    Literal { name: String, value: Vec<u8> },
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Statement {
    Block {
        code: Vec<Statement>,
    },
    FunctionDefinition {
        name:      String,
        arguments: Vec<String>,
        returns:   Vec<String>,
        code:      Vec<Statement>,
    },
    VariableDeclaration {
        variables: Vec<String>,
        value:     Option<Expression>,
    },
    Assignment {
        variables: Vec<String>,
        value:     Expression,
    },
    If {
        condition: Expression,
        code:      Vec<Statement>,
    },
    Expression {
        expression: Expression,
    },
    Switch {
        condition: Expression,
        cases:     Vec<SwitchCase>,
    },
    ForLoop {
        pre:       Vec<Statement>,
        condition: Expression,
        post:      Vec<Statement>,
        body:      Vec<Statement>,
    },
    Break,
    Continue,
    Leave,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Expression {
    FunctionCall {
        name:      String,
        arguments: Vec<Expression>,
    },
    Identifier(String),
    Literal(U256),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SwitchCase {
    Case { value: U256, code: Vec<Statement> },
    Default { code: Vec<Statement> },
}
