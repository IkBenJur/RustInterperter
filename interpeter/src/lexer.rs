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

}
