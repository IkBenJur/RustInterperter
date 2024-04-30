use crate::token::Token;
use std::char;

pub struct Lexer {
    input: String,
    position: Option<u32>,
    read_position: Option<u32>,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: None,
            read_position: None,
            ch: None,
        };
        lexer.read_char();
        return lexer;
    }

    fn read_char(&mut self) {
        if self.position.is_none() {
            self.position = Some(0);
            self.read_position = Some(0);
        }

        if let Some(read_position) = self.read_position {
            if read_position >= self.input.len() as u32 {
                self.ch = None;
            } else {
                if let Some(read_position) = self.read_position {
                    let new_char = self.input.chars().nth(read_position as usize).unwrap();
                    self.ch = Some(new_char);
                }

                self.position = self.read_position;
                self.read_position = Some(self.read_position.unwrap() + 1);
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        if let Some(read_position) = self.read_position {
            if read_position >= self.input.len() as u32 {
                return None;
            } else {
                return Some(self.input.chars().nth(read_position as usize).unwrap());
            }
        }

        return None;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_integer(&mut self) -> Option<String> {
        if let Some(start_position) = self.position {
            while let Some('0'..='9') = self.ch {
                self.read_char();
            }

            return Some(
                self.input[start_position as usize..self.position.unwrap() as usize].to_string(),
            );
        };

        return None;
    }

    fn read_identifier(&mut self) -> Option<String> {
        if let Some(start_position) = self.position {
            while let Some('a'..='z' | 'A'..='Z' | '_') = self.ch {
                self.read_char();
            }

            return Some(
                self.input[start_position as usize..self.position.unwrap() as usize].to_string(),
            );
        };

        return None;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;

        self.skip_whitespace();

        if let Some(char_literal) = self.ch {
            if let Some(t) = Token::from_char(char_literal) {
                token = t
            } else {
                match char_literal {
                    '=' => {
                        if let Some('=') = self.peek_char() {
                            self.read_char();
                            token = Token::EQ
                        } else {
                            token = Token::ASSIGN
                        }
                    }
                    '!' => {
                        if let Some('=') = self.peek_char() {
                            self.read_char();
                            token = Token::NOTEQ
                        } else {
                            token = Token::BANG
                        }
                    }
                    keyword_char => match keyword_char {
                        'a'..='z' | 'A'..='Z' | '_' => match self.read_identifier() {
                            None => {
                                return {
                                    println!("Unrecognized keyword char: {:?}", keyword_char);
                                    Token::ILLEGAL
                                };
                            }
                            Some(token_literal) => return Token::from_identifier(token_literal),
                        },
                        '0'..='9' => match self.read_integer() {
                            None => {
                                return {
                                    println!("Unrecognized number char: {:?}", keyword_char);
                                    Token::ILLEGAL
                                };
                            }
                            Some(interger) => return Token::from_interger_string(interger),
                        },
                        _ => {
                            return {
                                println!("Unrecognized char: {:?}", char_literal);
                                Token::ILLEGAL
                            };
                        }
                    },
                }
            }
        } else {
            token = Token::EOF
        }

        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_new_lexer() {
        let test_lexer = Lexer::new("{}".to_string());

        assert_eq!(test_lexer.input, "{}");
        assert_eq!(test_lexer.position, Some(0));
        assert_eq!(test_lexer.read_position, Some(1));
        assert_eq!(test_lexer.ch, Some('{'))
    }

    #[test]
    fn test_lexer_next_token() {
        let input = "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
          x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
    
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;"
            .to_string();

        let expected_types: Vec<Token> = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::INT(String::from("5")),
            Token::LT,
            Token::INT(String::from("10")),
            Token::GT,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(String::from("5")),
            Token::LT,
            Token::INT(String::from("10")),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(String::from("10")),
            Token::EQ,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::INT(String::from("10")),
            Token::NOTEQ,
            Token::INT(String::from("9")),
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut test_lexer = Lexer::new(input.clone());

        for i in 0..expected_types.len() {
            let t = test_lexer.next_token();

            println!("Token num: {}. Token: {:?}", i, t);

            assert_eq!(t, expected_types[i]);
        }
    }
}
