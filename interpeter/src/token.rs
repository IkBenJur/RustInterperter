impl Token {
    pub fn from_char(char_literal: char) -> Option<Self> {
        match char_literal {
            ';' => return Some(Token::SEMICOLON),
            '(' => return Some(Token::LPAREN),
            ')' => return Some(Token::RPAREN),
            '{' => return Some(Token::LBRACE),
            '}' => return Some(Token::RBRACE),
            ',' => return Some(Token::COMMA),
            '+' => return Some(Token::PLUS),
            '-' => return Some(Token::MINUS),
            '/' => return Some(Token::SLASH),
            '*' => return Some(Token::ASTERISK),
            '<' => return Some(Token::LT),
            '>' => return Some(Token::GT),
            _ => None,
        }
    }
    pub fn from_identifier(identifier: String) -> Self {
        match identifier.as_str() {
            "let" => return Self::LET,
            "fn" => return Self::FUNCTION,
            "true" => return Self::TRUE,
            "false" => return Self::FALSE,
            "if" => return Self::IF,
            "else" => return Self::ELSE,
            "return" => return Self::RETURN,
            _ => return Self::IDENT(identifier),
        }
    }

    pub fn from_interger_string(interger: String) -> Self {
        return Token::INT(interger.parse().unwrap());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String), // add, foobar, x, y, ...
    INT(u128),     // 1343456

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,
    EQ,
    NOTEQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}
