#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(Expresion, Expresion),
    Return(Expresion),
    Expression(Expresion),
}

#[derive(PartialEq, Debug)]
pub enum Expresion {
    Identifer(String),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
