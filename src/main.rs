use crate::lexer::Lexer;

mod lexer;

fn main() {
    let input = "(  + 2   (*  3  4)  nil abc true)";

    let my_lexer = Lexer::new(input.to_string());

    let tokens:Vec<_> = my_lexer.collect();

    for token in tokens {
        dbg!(token);
    }
}