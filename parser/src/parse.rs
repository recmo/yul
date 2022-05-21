use crate::SyntaxNode;
use rowan::{GreenNode, NodeOrToken};

#[derive(Clone)]
pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn new(green_node: GreenNode) -> Self {
        Self { green_node }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    /// Recover source code from syntax tree.
    pub fn unparse(&self) -> String {
        let mut result = String::with_capacity(self.green_node.text_len().into());
        unparse_recurse(&mut result, &self.green_node);
        result
    }

    pub fn debug_tree(&self) -> String {
        let mut formatted = format!("{:#?}", self);
        formatted.pop(); // Cut trailing newline
        formatted
    }
}

impl core::fmt::Debug for Parse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.syntax_node().fmt(f)
    }
}

impl serde::Serialize for Parse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.syntax_node().serialize(serializer)
    }
}

fn unparse_recurse(result: &mut String, node: &GreenNode) {
    for child in node.children() {
        match child {
            NodeOrToken::Node(node) => unparse_recurse(result, node),
            NodeOrToken::Token(token) => result.push_str(token.text().as_str()),
        }
    }
}
