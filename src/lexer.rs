use anyhow::Result;
#[derive(Debug, PartialEq)]
enum Token {
    EoF,          // End of input

    LParen,       // Left parenthesis "("
    RParen,       // Right parenthesis ")"
    LSquiggly,    // Left squiggly brace "{"
    RSquiggly,    // Right squiggly brace "}"
    LSquare,      // Left square bracket "["
    RSquare,      // Right square bracket "]"

    Ident(String), // Symbols and identifiers

    Int(String),   // Integer literals "123"
    Float(String), // Floating-point literals "123.456"
    Str(String),   // String literals "Hello world"

    Keyword(String), // Keywords (e.g., :keyword)
    Quote,         // Quote character (')
    Dot,           // Dot (.) for cons cells
    True,          // Boolean true (t)
    False,         // Boolean false (nil)
    Nil,           // Nil, often used for empty lists
    Comment(String), // Comments

    Char(String),  // Character literals (e.g., #\a, #\newline)
}

pub struct Lexer {
    position: usize,
    current_char: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            current_char: 0,
            input: input.into_bytes(),
        };
        return lex;
    }


    fn next(&mut self) -> Result<Token> {
        self.skip_whitespace();
        
        let token = match self.current_char {




        };
        
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = 0;
        } else {
            self.current_char = self.input[self.position];
        }
        self.advance();
    }
    fn skip_whitespace(&mut self) {
        while self.current_char.is_ascii_whitespace() {
            self.read_char();
        }
    }
    fn advance(&mut self) {self.position += 1;}


}
