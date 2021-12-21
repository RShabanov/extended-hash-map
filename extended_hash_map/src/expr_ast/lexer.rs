use std::{
    iter::Peekable,
    str::Chars
};
use super::token::{
    Token,
    literal::Literal,
    op::OpKind
};

fn is_ignore_char(ch: char) -> bool {
    ch.is_whitespace() || "()".contains(ch)
}

#[derive(Debug)]
pub enum LexerErr {}

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    current_char: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn next(&mut self) -> Result<Token, LexerErr> {
        while let Some(&next_char) = self.current_char.peek() {
            if is_ignore_char(next_char) {
                self.skip();
                continue;
            }

            if let Some(op) = self.op_kind(next_char) {
                return Ok(Token::Op(op))
            } else if next_char.is_ascii_digit() {
                return Ok(Token::Literal(self.number()))
            } else {
                self.skip_delim();
                // self.current_char.next();
                return Ok(Token::Delim)
            }
        }

        Ok(Token::Eof)
    }

    fn number(&mut self) -> Literal {
        let mut int_part = self.integer();

        match self.current_char.next_if_eq(&'.') {
            Some(dot) => {
                int_part.push(dot);
                Literal::Float(int_part + &self.integer())
            },
            None => Literal::Integer(int_part)
        }
    }

    fn integer(&mut self) -> String {
        let mut int = String::new();

        while let Some(ch) = self.current_char
            .next_if(|&ch| ch.is_ascii_digit()) {
            int.push(ch);
        }
        int
    }

    fn skip(&mut self) {
        while let Some(_) = self.current_char
            .next_if(|&ch| is_ignore_char(ch)) {}
    }

    fn skip_delim(&mut self) {
        while let Some(_) = self.current_char
            .next_if(|&ch| !is_ignore_char(ch)
                            && !ch.is_ascii_digit()
                            && !"<>=>".contains(ch)) {}
    }

    fn op_kind(&mut self, ch: char) -> Option<OpKind> {
        match ch {
            '=' => Some(OpKind::Eq),
            '>' => {
                self.current_char.next();
                match self.current_char.next_if_eq(&'=') {
                    Some(_) => Some(OpKind::Ge),
                    None => Some(OpKind::Gt)
                }
            },
            '<' => {
                self.current_char.next();
                match self.current_char.next_if(|&ch| "=>".contains(ch)) {
                    Some(ch) => return Some(
                        if ch == '=' { OpKind::Le }
                        else { OpKind::Ne }
                    ),
                    None => return Some(OpKind::Lt)
                }
            },
            _ => None
        }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(text: &'a str) -> Self {
        Self { current_char: text.chars().peekable() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exprs_and_tokens() -> (Vec<&'static str>, Vec<Vec<Token>>) {
        (
            vec![
                "(>= 4)",
                "<3.5",
                ">0, <10",
                "<5, >=5, >=3",
                "<5abc>=5 &|c>=3",
                "a>=2"
            ],
            vec![
                vec![
                    Token::Op(OpKind::Ge),
                    Token::Literal(Literal::Integer(String::from("4")))
                ],
                vec![
                    Token::Op(OpKind::Lt),
                    Token::Literal(Literal::Float(String::from("3.5")))
                ],
                vec![
                    Token::Op(OpKind::Gt),
                    Token::Literal(Literal::Integer(String::from("0"))),
                    Token::Delim,
                    Token::Op(OpKind::Lt),
                    Token::Literal(Literal::Integer(String::from("10")))
                ],
                vec![
                    Token::Op(OpKind::Lt),
                    Token::Literal(Literal::Integer(String::from("5"))),
                    Token::Delim,
                    Token::Op(OpKind::Ge),
                    Token::Literal(Literal::Integer(String::from("5"))),
                    Token::Delim,
                    Token::Op(OpKind::Ge),
                    Token::Literal(Literal::Integer(String::from("3"))),
                ],
                vec![
                    Token::Op(OpKind::Lt),
                    Token::Literal(Literal::Integer(String::from("5"))),
                    Token::Delim,
                    Token::Op(OpKind::Ge),
                    Token::Literal(Literal::Integer(String::from("5"))),
                    Token::Delim,
                    Token::Op(OpKind::Ge),
                    Token::Literal(Literal::Integer(String::from("3"))),
                ],
                vec![
                    Token::Delim,
                    Token::Op(OpKind::Ge),
                    Token::Literal(Literal::Integer(String::from("2")))
                ],
            ]
        )
    }

    #[test]
    fn create_from() {
        let lexer = Lexer::from(">= 4");
        let lexer_str = "Lexer { current_char: Peekable { iter: Chars(['>', '=', ' ', '4']), peeked: None } }";

        assert_eq!(lexer_str, format!("{:?}", lexer));
    }

    #[test]
    fn tokenize() {
        let (exprs, expr_tokens) = exprs_and_tokens();

        for (expr, tokens) in exprs.iter().zip(expr_tokens.iter()) {
            let mut lexer = Lexer::from(*expr);

            println!("LEXER: {:?}", lexer);
            println!("LEN: {}", tokens.len());
            for token in tokens {
                assert_eq!(lexer.next().unwrap(), *token);
            }
        }
    }
}