pub struct Lexer {
    input: String,
    position: Option<u32>,
    read_position: Option<u32>,
    ch: Option<char>,
}
