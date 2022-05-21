use crate::SyntaxNode;

pub trait AstNode {
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnDef {
    syntax: SyntaxNode,
}

impl AstNode for FnDef {
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match kind {
            FN => Some(FnDef { syntax }),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl FnDef {
    pub fn param_list(&self) -> Option<ParamList> {
        self.syntax.children().find_map(ParamList::cast)
    }

    pub fn ret_type(&self) -> Option<RetType> {
        self.syntax.children().find_map(RetType::cast)
    }

    pub fn body(&self) -> Option<BlockExpr> {
        self.syntax.children().find_map(BlockExpr::cast)
    }
    // ...
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssocItem {
    FnDef(FnDef),
    TypeAliasDef(TypeAliasDef),
    ConstDef(ConstDef),
}

impl AstNode for AssocItem {
    ...
}

trait HasVisibility: AstNode {
    fn visibility(&self) -> Option<Visibility>;
}

impl HasVisbility for FnDef {
    fn visibility(&self) -> Option<Visibility> {
        self.syntax.children().find_map(Visibility::cast)
    }
}