//Rewrite token to just be token enum.
//Enum values will be of something like Token::Ident(string)
//And Token::asign wont have anything because asign doesnt hold a varaible etc.

use crate::{
    ast::{Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lexer = Lexer::new(input);
        let cur_token = lexer.next_token();
        let next_token = lexer.next_token();

        Parser {
            lexer,
            cur_token,
            next_token,
        }
    }

    fn advance_token(&mut self) {
        self.cur_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn cur_token_is(&self, token: Token) -> bool {
        return &self.cur_token == &token;
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.cur_token_is(token) {
            self.advance_token();
            return true;
        } else {
            return false;
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, &'static str> {
        self.advance_token();

        let identifier = match &self.cur_token {
            Token::IDENT(string) => string.to_owned(),
            _ => return Err("No identiefer found after let statement"),
        };

        self.advance_token();

        if !self.expect_peek(Token::ASSIGN) {
            return Err("No equal sign found after let identiefer");
        }

        while !self.cur_token_is(Token::SEMICOLON) {
            self.advance_token();
        }

        return Ok(Statement::Let(identifier));
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.cur_token != Token::EOF {
            let statement = match self.cur_token {
                Token::LET => match self.parse_let_statement() {
                    Ok(let_statement) => let_statement,
                    Err(e) => panic!("{}", e),
                },
                _ => todo!("Not yet done"),
            };

            self.advance_token();
            statements.push(statement);
        }

        return Program {
            statements: statements,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::Statement;

    #[test]
    fn test_parse_program() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "
        .to_string();

        let mut parser = Parser::new(input);
        let program = parser.parse_program();

        assert_eq!(3, program.statements.len());

        let expected_identifiers = vec![
            Statement::Let(String::from("x")),
            Statement::Let(String::from("y")),
            Statement::Let(String::from("foobar")),
        ];

        for i in 0..=expected_identifiers.len() - 1 {
            assert_eq!(program.statements[i], expected_identifiers[i]);
        }
    }
}
