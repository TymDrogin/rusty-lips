mod lexer;
fn main() {
    // `()` can be used when no completer is required
    let input = "(+ 2 (* 3 4))";

    let mut lexer = lexer::Lexer::new(input.to_string());
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}

