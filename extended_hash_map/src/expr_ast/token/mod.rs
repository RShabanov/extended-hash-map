pub mod literal;
pub mod op;

use op::OpKind;
use literal::Literal;

#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(Literal),
    Op(OpKind),
    Delim,
    Eof
}