use super::token::{
    op::OpKind, 
    literal::Literal
};

#[derive(PartialEq, Eq, Debug)]
pub struct BinOp {
    pub lhs: Literal,
    pub op: OpKind,
    pub rhs: Literal
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Literal(Literal),
    BinOp(BinOp)
}

#[derive(Debug, Default)]
pub struct Tree {
    pub root: Vec<Node>
}

impl Tree {
    pub fn len(&self) -> usize {
        self.root.len()
    }
}