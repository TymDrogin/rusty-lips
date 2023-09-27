use crate::lexer::Lexer;

mod lexer;

fn main() {
    let input = "(  123   456 789   )";

    let my_lexer = Lexer::new(input.to_string());

    let tokens:Vec<_> = my_lexer.collect();

    for token in tokens {
        dbg!(token);
    }
}