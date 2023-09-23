use anyhow::Result;
use std::option;
#[derive(Debug, PartialEq)]
enum Token {
    EoF,                // End of file

    // Parentheses and Brackets
    LParen,             // Left parenthesis
    RParen,             // Right parenthesis
    LSquiggly,          // Left brace
    RSquiggly,          // Right brace
    LSquare,            // Left square bracket
    RSquare,            // Right square bracket

    // Literals
    Integer(i64),       // Integer literals
    Float(f64),         // Floating-point literals
    String(String),     // String literals

    //Symbols, ident and keyword
    Symbol(String),     // For any special symbol or combinations like +, -, =, >=, ~@, etc
    Ident(String),      // Identifiers such as function names
    Keyword(String),    // Keywords

    // Booleans and Nil
    True,               // Boolean true
    False,              // Boolean false
    Nil,                // Nil (often used for empty lists)

    Comment(String),    // Comments
}

#[allow(unused)]
pub struct Lexer {
    position: usize,
    curr_char: Option<u8>,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            curr_char: None,
            input: input.into_bytes(),
        };
        lex.read_char();
        return lex;
    }
    pub fn tokenize() -> Vec<Token> {
        todo!();
    }

    fn next_token() -> Token {
        todo!();
    }





    fn parse_number() -> Token {
        todo!()
    }
    fn parse_ident() -> Token {
        todo!()
    }

    fn read_char(&mut self) {
        self.curr_char = self.peek();
        self.advance();
    }
    fn peek(&mut self) -> Option<u8> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        }
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn skip_whitespace(&mut self) {
        while self.curr_char.is_ascii_whitespace() || self.curr_char.unwrap() == b',' {

        }
    }
















}