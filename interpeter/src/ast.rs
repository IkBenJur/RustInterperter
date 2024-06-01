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
    Plus,
    Multiply,
    Divide,
    Gt,
    Lt,
    Equals,
    NotEquals,
    Not,
}

impl From<Token> for Operator {
    fn from(value: Token) -> Self {
        match value {
            Token::MINUS => return Operator::Minus,
            Token::PLUS => return Operator::Plus,
            Token::ASTERISK => return Operator::Multiply,
            Token::SLASH => return Operator::Divide,
            Token::GT => return Operator::Gt,
            Token::LT => return Operator::Lt,
            Token::EQ => return Operator::Equals,
            Token::NOTEQ => return Operator::NotEquals,
            Token::BANG => return Operator::Not,
            _ => unimplemented!("Token not valid to parse into operator {:?}", value),
        }
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    Lessgreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

impl From<Token> for Precedence {
    fn from(value: Token) -> Self {
        match value {
            Token::LT | Token::GT => return Precedence::Lessgreater,
            Token::EQ | Token::NOTEQ => return Precedence::Equals,
            Token::PLUS | Token::MINUS => return Precedence::Sum,
            Token::SLASH | Token::ASTERISK => return Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}
