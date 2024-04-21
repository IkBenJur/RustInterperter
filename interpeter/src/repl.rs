use std::io::Stdin;

use crate::{lexer::Lexer, token::TokenType};

pub fn start(in_reader: Stdin) {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match in_reader.read_line(&mut buffer) {
            Ok(bytes) => {
                let mut lexer = Lexer::new(buffer.clone());

                let mut token = lexer.next_token();
                while token.t_type != TokenType::EOF {
                    println!("Type: {:?}, Literal: {}", token.t_type, token.literal);
                    token = lexer.next_token();
                }
            }
            Err(error) => println!("error {error}"),
        }
    }
}
