use crate::lexer::{Lexer, Token};

mod lexer;

fn main() {
    let input = "(+2(*3 4) nil abc true)";

    let my_lexer = Lexer::new(input.to_string());

    let tokens:Vec<_> = my_lexer.collect()
    tokens.filter(|x| x == Token::Ident);

    for token in tokens {
        dbg!(token);
    }
}