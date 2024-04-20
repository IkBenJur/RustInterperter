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
