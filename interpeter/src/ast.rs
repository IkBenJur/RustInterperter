#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(String)
}

#[derive(PartialEq, Debug)]
pub enum Expresion {
    Identifer(String)
}

pub struct Program {
    pub statements: Vec<Statement>,
}
