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

#[derive(Debug, Default)]
pub struct Tree {
    pub root: Vec<BinOp>
}