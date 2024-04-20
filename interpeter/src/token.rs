pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, token_literal: String) -> Self {
        Self {
            t_type: token_type,
            literal: token_literal,
        }
    }

    pub fn new_from_char(token_type: TokenType, char_literal: char) -> Self {
        Self {
            t_type: token_type,
            literal: char_literal.to_string(),
        }
    }

    pub fn from_identifier(identifier: String) -> Self {
        match identifier.as_str() {
            "let" => return Self::new(TokenType::LET, identifier),
            "fn" => return Self::new(TokenType::FUNCTION, identifier),
            _ => return Self::new(TokenType::IDENT, identifier),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT,   // 1343456

    // Operators
    ASSIGN,
    PLUS,

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
}
