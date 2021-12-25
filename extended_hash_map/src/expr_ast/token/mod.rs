pub mod literal;
pub mod op;

use literal::Literal;
use op::OpKind;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Literal(Literal),
    Op(OpKind),
    Delim,
    Eof,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        *self == Token::Eof
    }

    pub fn is_delim(&self) -> bool {
        *self == Token::Delim
    }

    pub fn is_op(&self) -> bool {
        match *self {
            Token::Op(_) => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match *self {
            Token::Literal(_) => true,
            _ => false,
        }
    }
}
