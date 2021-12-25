use super::{
    lexer::Lexer,
    node::{BinOp, Node, Tree},
    token::{literal::Literal, op::OpKind, Token},
};

#[derive(Debug, PartialEq)]
pub enum ParserErr {
    InvalidToken(String),
    InvalidExpr,
    TokenMismatch,
    Undefined,
}

#[derive(Debug)]
pub struct Parser<'a> {
    current_token: Token,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(&mut self, text: &'a str) -> Result<Tree, ParserErr> {
        self.lexer.set(text);
        self.next_token()?;

        let mut tree = Tree::default();

        while !self.current_token.is_eof() {
            if self.current_token.is_delim() {
                self.next_token()?;

                if self.current_token.is_eof() {
                    break;
                }
            }

            tree.root.push(Node::Literal(self.literal()?));

            self.next_token()?;
        }

        Ok(tree)
    }

    pub fn parse_condition(&mut self, text: &'a str) -> Result<Tree, ParserErr> {
        self.lexer.set(text);
        self.next_token()?;

        let mut tree = Tree::default();

        while !self.current_token.is_eof() {
            if self.current_token.is_delim() {
                self.next_token()?;

                if self.current_token.is_eof() {
                    break;
                }
            }

            tree.root.push(Node::BinOp(self.bin_op()?));
        }

        Ok(tree)
    }

    fn next_token(&mut self) -> Result<(), ParserErr> {
        match self.lexer.next() {
            Ok(token) => Ok(self.current_token = token),
            Err(_) => Err(ParserErr::Undefined),
        }
    }

    fn bin_op(&mut self) -> Result<BinOp, ParserErr> {
        let mut bin_op = BinOp {
            lhs: Literal::Integer(String::new()),
            op: OpKind::Eq,
            rhs: Literal::Integer(String::new()),
        };

        match self.current_token {
            Token::Op(op) => bin_op.op = op,
            _ => return Err(ParserErr::TokenMismatch),
        }

        self.next_token()?;

        bin_op.rhs = self.literal()?;

        self.next_token()?;

        Ok(bin_op)
    }

    fn literal(&mut self) -> Result<Literal, ParserErr> {
        match self.current_token {
            Token::Literal(ref mut lit) => {
                let mut temp_lit = Literal::Integer(String::new());
                std::mem::swap(&mut temp_lit, lit);

                Ok(temp_lit)
            }
            _ => return Err(ParserErr::TokenMismatch),
        }
    }
}

impl Default for Parser<'_> {
    fn default() -> Self {
        Self {
            current_token: Token::Eof,
            lexer: Lexer::from("\0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn condition_exprs_and_trees() -> (Vec<&'static str>, Vec<Tree>) {
        (
            vec![
                "( >= 4)",
                "<3.5",
                ">0, <10",
                "<5, >=5, >=3",
                "<5abc>=5 &|c>=3",
                "a>=2",
                " < 5 >=5 >= 3",
            ],
            vec![
                Tree {
                    root: vec![Node::BinOp(BinOp {
                        lhs: Literal::Integer(String::new()),
                        op: OpKind::Ge,
                        rhs: Literal::Integer("4".to_string()),
                    })],
                },
                Tree {
                    root: vec![Node::BinOp(BinOp {
                        lhs: Literal::Integer(String::new()),
                        op: OpKind::Lt,
                        rhs: Literal::Float("3.5".to_string()),
                    })],
                },
                Tree {
                    root: vec![
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Gt,
                            rhs: Literal::Integer("0".to_string()),
                        }),
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Lt,
                            rhs: Literal::Integer("10".to_string()),
                        }),
                    ],
                },
                Tree {
                    root: vec![
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Lt,
                            rhs: Literal::Integer("5".to_string()),
                        }),
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Ge,
                            rhs: Literal::Integer("5".to_string()),
                        }),
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Ge,
                            rhs: Literal::Integer("3".to_string()),
                        }),
                    ],
                },
                Tree {
                    root: vec![
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Lt,
                            rhs: Literal::Integer("5".to_string()),
                        }),
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Ge,
                            rhs: Literal::Integer("5".to_string()),
                        }),
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Ge,
                            rhs: Literal::Integer("3".to_string()),
                        }),
                    ],
                },
                Tree {
                    root: vec![Node::BinOp(BinOp {
                        lhs: Literal::Integer(String::new()),
                        op: OpKind::Ge,
                        rhs: Literal::Integer("2".to_string()),
                    })],
                },
                Tree {
                    root: vec![
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Lt,
                            rhs: Literal::Integer("5".to_string()),
                        }),
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Ge,
                            rhs: Literal::Integer("5".to_string()),
                        }),
                        Node::BinOp(BinOp {
                            lhs: Literal::Integer(String::new()),
                            op: OpKind::Ge,
                            rhs: Literal::Integer("3".to_string()),
                        }),
                    ],
                },
            ],
        )
    }

    fn exprs_and_trees() -> (Vec<&'static str>, Vec<Tree>) {
        (
            vec!["(5, 5)", "value1"],
            vec![
                Tree {
                    root: vec![
                        Node::Literal(Literal::Integer(String::from("5"))),
                        Node::Literal(Literal::Integer(String::from("5"))),
                    ],
                },
                Tree {
                    root: vec![Node::Literal(Literal::Integer(String::from("1")))],
                },
            ],
        )
    }

    #[test]
    fn create() {
        let mut parser = Parser::new();
        assert_eq!(parser.current_token, Token::Eof);

        parser = Parser::default();
        assert_eq!(parser.current_token, Token::Eof);
    }

    #[test]
    fn parse_condition() {
        let mut parser = Parser::new();
        let (exprs, trees) = condition_exprs_and_trees();

        for (expr, tree) in exprs.iter().zip(trees.iter()) {
            assert_eq!(parser.parse_condition(expr).unwrap().root, tree.root);
        }
    }

    #[test]
    fn parse() {
        let mut parser = Parser::new();
        let (exprs, trees) = exprs_and_trees();

        for (expr, tree) in exprs.iter().zip(trees.iter()) {
            assert_eq!(parser.parse(expr).unwrap().root, tree.root);
        }
    }

    #[test]
    #[should_panic]
    fn parse_invalid_condition() {
        let mut parser = Parser::new();

        match parser.parse_condition(">,.2") {
            Ok(_) => (),
            Err(_) => panic!(),
        }
    }
}
