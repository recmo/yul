use crate::{Node, Token};
use num_traits::{FromPrimitive, ToPrimitive};
use rowan;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SyntaxKind {
    Token(Token),
    Node(Node),
}

impl core::fmt::Debug for SyntaxKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            SyntaxKind::Token(token) => token.fmt(f),
            SyntaxKind::Node(node) => node.fmt(f),
        }
    }
}

impl ToPrimitive for SyntaxKind {
    fn to_u64(&self) -> Option<u64> {
        match self {
            SyntaxKind::Token(token) => token.to_u64().map(|n| 2 * n),
            SyntaxKind::Node(node) => node.to_u64().map(|n| 2 * n + 1),
        }
    }

    fn to_i64(&self) -> Option<i64> {
        match self {
            SyntaxKind::Token(token) => token.to_i64().map(|n| 2 * n),
            SyntaxKind::Node(node) => node.to_i64().map(|n| 2 * n + 1),
        }
    }
}

impl FromPrimitive for SyntaxKind {
    fn from_i64(n: i64) -> std::option::Option<Self> {
        match (n & 1, n / 2) {
            (0, n) => Token::from_i64(n).map(SyntaxKind::Token),
            (1, n) => Node::from_i64(n).map(SyntaxKind::Node),
            _ => unreachable!(),
        }
    }

    fn from_u64(n: u64) -> std::option::Option<Self> {
        match (n & 1, n / 2) {
            (0, n) => Token::from_u64(n).map(SyntaxKind::Token),
            (1, n) => Node::from_u64(n).map(SyntaxKind::Node),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YulLanguage {}

impl rowan::Language for YulLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<YulLanguage>;
