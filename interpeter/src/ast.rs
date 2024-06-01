use crate::token::Token;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(Expresion, Expresion),
    Return(Expresion),
    Expression(Expresion),
}

#[derive(PartialEq, Debug)]
pub enum Expresion {
    Identifer(String),
    Interger(u64),
    Prefix(Operator, Box<Expresion>),
}

#[derive(PartialEq, Debug)]
pub enum Operator {
    Minus,
    Not,
}

impl From<Token> for Operator {
    fn from(value: Token) -> Self {
        match value {
            Token::MINUS => return Operator::Minus,
            Token::BANG => return Operator::Not,
            _ => unimplemented!("Token not valid to parse into operator {:?}", value),
        }
    }
}

pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    Lessgreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

pub struct Program {
    pub statements: Vec<Statement>,
}
