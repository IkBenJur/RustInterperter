use crate::token::{Token, TokenType};

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
            while let Some(ch) = self.ch {
                if ('0'..='9').contains(&ch) {
                    self.read_char();
                } else {
                    break;
                };
            }

            return Some(
                self.input[start_position as usize..self.position.unwrap() as usize].to_string(),
            );
        };

        return None;
    }

    fn read_indetifer(&mut self) -> Option<String> {
        if let Some(start_position) = self.position {
            while let Some(ch) = self.ch {
                if ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || '_' == ch {
                    self.read_char();
                } else {
                    break;
                };
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

        match self.ch {
            None => {
                token = Token {
                    t_type: TokenType::EOF,
                    literal: "".to_string(),
                }
            }
            Some(char_literal) => match char_literal {
                '=' => token = Token::new_from_char(TokenType::ASSIGN, char_literal),
                ';' => token = Token::new_from_char(TokenType::SEMICOLON, char_literal),
                '(' => token = Token::new_from_char(TokenType::LPAREN, char_literal),
                ')' => token = Token::new_from_char(TokenType::RPAREN, char_literal),
                ',' => token = Token::new_from_char(TokenType::COMMA, char_literal),
                '+' => token = Token::new_from_char(TokenType::PLUS, char_literal),
                '{' => token = Token::new_from_char(TokenType::LBRACE, char_literal),
                '}' => token = Token::new_from_char(TokenType::RBRACE, char_literal),
                keyword_char => match keyword_char {
                    'a'..='z' | 'A'..='Z' | '_' => match self.read_indetifer() {
                        None => {
                            return {
                                println!("Unrecognized keyword char: {:?}", keyword_char);
                                Token::new_from_char(TokenType::ILLEGAL, keyword_char)
                            };
                        }
                        Some(token_literal) => return Token::from_identifier(token_literal),
                    },
                    '0'..='9' => match self.read_integer() {
                        None => {
                            return {
                                println!("Unrecognized number char: {:?}", keyword_char);
                                Token::new_from_char(TokenType::ILLEGAL, keyword_char)
                            }
                        }
                        Some(interger) => return Token::new(TokenType::INT, interger),
                    },
                    _ => {
                        token = {
                            println!("Unrecognized char: {:?}", char_literal);
                            Token::new_from_char(TokenType::ILLEGAL, char_literal)
                        }
                    }
                },
            },
        }

        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

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
        
        let result = add(five, ten);"
            .to_string();

        struct ExpectedToken {
            expected_type: TokenType,
            expected_literal: String,
        }

        let expected_types: Vec<ExpectedToken> = vec![
            ExpectedToken {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "five".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "ten".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "add".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::FUNCTION,
                expected_literal: "fn".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "x".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "y".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::RPAREN,
                expected_literal: ")".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::LBRACE,
                expected_literal: "{".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "x".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::PLUS,
                expected_literal: "+".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "y".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::RBRACE,
                expected_literal: "}".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "result".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "add".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "five".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::IDENT,
                expected_literal: "ten".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::RPAREN,
                expected_literal: ")".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            ExpectedToken {
                expected_type: TokenType::EOF,
                expected_literal: "".to_string(),
            },
        ];

        let mut test_lexer = Lexer::new(input.clone());

        for i in 0..expected_types.len() {
            let t = test_lexer.next_token();

            assert_eq!(t.literal, expected_types[i].expected_literal);
            assert_eq!(t.t_type, expected_types[i].expected_type);
        }
    }
}
