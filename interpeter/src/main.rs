mod lexer;
mod token;
mod repl;

fn main() {
    repl::start(std::io::stdin());
}
