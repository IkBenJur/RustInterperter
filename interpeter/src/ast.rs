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
}
#[derive(PartialEq, Debug)]
pub enum Operator {
    Minus,
    Not,
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
