//Rewrite token to just be token enum.
//Enum values will be of something like Token::Ident(string)
//And Token::asign wont have anything because asign doesnt hold a varaible etc.

use crate::{
    ast::{Expresion, Operator, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    next_token: Token,
}

type ParseError = String;

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

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.cur_token != Token::EOF {
            let statement = match self.cur_token {
                Token::LET => match self.parse_let_statement() {
                    Ok(let_statement) => let_statement,
                    Err(e) => panic!("{}", e),
                },
                Token::RETURN => match self.parse_return_statement() {
                    Ok(return_statement) => return_statement,
                    Err(e) => panic!("{}", e),
                },
                _ => match self.parse_expression() {
                    Ok(expresion) => expresion,
                    Err(e) => panic!("{}", e),
                },
            };

            self.advance_token();
            statements.push(statement);
        }

        return Program {
            statements: statements,
        };
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance_token();

        let identifier = match &self.cur_token {
            Token::IDENT(string) => string.to_owned(),
            _ => {
                return Err(format!(
                    "No identiefer found after let statement found: {:?}",
                    self.cur_token
                ))
            }
        };

        self.advance_token();

        if !self.expect_peek(Token::ASSIGN) {
            return Err(format!(
                "No equal sign found after let identiefer found: {:?}",
                self.cur_token
            ));
        }

        while !self.cur_token_is(Token::SEMICOLON) {
            self.advance_token();
        }

        return Ok(Statement::Let(
            Expresion::Identifer(identifier),
            Expresion::Identifer(String::from("")),
        ));
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.advance_token();

        let expresion = match &self.cur_token {
            Token::INT(num) => num.to_owned(),
            _ => return Err(format!("No experssion found, found: {:?}", self.cur_token)),
        };

        while !self.cur_token_is(Token::SEMICOLON) {
            self.advance_token();
        }

        return Ok(Statement::Return(Expresion::Identifer(expresion)));
    }

    fn parse_expression(&mut self) -> Result<Statement, ParseError> {
        let expression = match &self.cur_token {
            Token::BANG => self.parse_prefix()?,
            Token::MINUS => self.parse_prefix()?,
            Token::IDENT(_) => self.parse_identifier()?,
            Token::INT(_) => self.parse_integer()?,
            _ => {
                return Err(format!(
                    "Non implemetned expression found {:?}",
                    &self.cur_token
                ))
            }
        };

        if self.next_token_is(Token::SEMICOLON) {
            self.advance_token()
        }

        return Ok(Statement::Expression(expression));
    }

    fn parse_identifier(&self) -> Result<Expresion, ParseError> {
        if let Token::IDENT(string) = &self.cur_token {
            return Ok(Expresion::Identifer(string.to_owned()));
        }

        return Err(format!(
            "No Identifier found whilst trying to parse identiefer found: {:?}",
            self.cur_token
        ));
    }

    fn parse_integer(&self) -> Result<Expresion, ParseError> {
        if let Token::INT(num_literal) = &self.cur_token {
            let num = match num_literal.to_owned().parse() {
                Ok(parsed_num) => Ok(Expresion::Interger(parsed_num)),
                Err(_) => Err(format!(
                    "Failed to parse number into Interger found: {:?}",
                    self.cur_token
                )),
            };

            return num;
        }

        return Err(format!(
            "No integer found whilst trying to parse integer found: {:?}",
            self.cur_token
        ));
    }

    fn parse_operator(&self) -> Operator {
        return Operator::from(self.cur_token.clone());
    }

    fn parse_prefix(&mut self) -> Result<Expresion, ParseError> {
        let left = self.parse_operator();

        self.advance_token();

        let right: Expresion = match self.parse_expression() {
            Ok(statement) => {
                if let Statement::Expression(expr) = statement {
                    expr
                } else {
                    return Err(format!(
                        "No expression statement found after prefix operator found: {:?}",
                        self.cur_token
                    ));
                }
            }
            Err(_) => {
                return Err(format!(
                    "No expression statement found after prefix operator found: {:?}",
                    self.cur_token
                ))
            }
        };

        return Ok(Expresion::Prefix(left, Box::new(right)));
    }

    fn cur_token_is(&self, token: Token) -> bool {
        return &self.cur_token == &token;
    }

    fn next_token_is(&self, token: Token) -> bool {
        return &self.next_token == &token;
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.cur_token_is(token) {
            self.advance_token();
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::{Expresion, Operator, Statement};

    #[test]
    fn test_let_statement_parse_program() {
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
            Statement::Let(
                Expresion::Identifer(String::from("x")),
                Expresion::Identifer(String::from("")),
            ),
            Statement::Let(
                Expresion::Identifer(String::from("y")),
                Expresion::Identifer(String::from("")),
            ),
            Statement::Let(
                Expresion::Identifer(String::from("foobar")),
                Expresion::Identifer(String::from("")),
            ),
        ];

        for i in 0..=expected_identifiers.len() - 1 {
            assert_eq!(program.statements[i], expected_identifiers[i]);
        }
    }

    #[test]
    fn test_return_statement_parse_program() {
        let input = "
        return 5;
        return 10;
        return 993322;
        "
        .to_string();

        let mut parser = Parser::new(input);
        let program = parser.parse_program();

        assert_eq!(3, program.statements.len());

        let expected_statements = vec![
            Statement::Return(Expresion::Identifer(String::from("5"))),
            Statement::Return(Expresion::Identifer(String::from("10"))),
            Statement::Return(Expresion::Identifer(String::from("993322"))),
        ];

        for i in 0..=expected_statements.len() - 1 {
            assert_eq!(program.statements[i], expected_statements[i]);
        }
    }

    #[test]
    fn test_expression_statement_parse_program() {
        let input = "foobar;
        5;"
        .to_string();

        let program = Parser::new(input).parse_program();

        let expected_statements = vec![
            Statement::Expression(Expresion::Identifer(String::from("foobar"))),
            Statement::Expression(Expresion::Interger(5)),
        ];

        assert_eq!(expected_statements.len(), program.statements.len());

        for i in 0..=expected_statements.len() - 1 {
            assert_eq!(program.statements[i], expected_statements[i]);
        }
    }

    #[test]
    fn test_prefix_expression_parse_program() {
        let input = "-5;
        !foobar;"
            .to_string();

        let program = Parser::new(input).parse_program();

        let expected_statements = vec![
            Statement::Expression(Expresion::Prefix(
                Operator::Minus,
                Box::new(Expresion::Interger(5)),
            )),
            Statement::Expression(Expresion::Prefix(
                Operator::Not,
                Box::new(Expresion::Identifer(String::from("foobar"))),
            )),
        ];

        assert_eq!(expected_statements.len(), program.statements.len());

        for i in 0..=expected_statements.len() - 1 {
            assert_eq!(program.statements[i], expected_statements[i]);
        }
    }
}
