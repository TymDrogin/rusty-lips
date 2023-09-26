use std::panic::resume_unwind;

#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum Token {
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
    Symbol(String),     // For any special symbol or combinations like +, -, =, >, ~@, etc
    Ident(String),      // Identifiers such as function names
    Keyword(String),    // Keywords

    // Booleans and Nil
    True,               // Boolean true
    False,              // Boolean false
    Nil,                // Nil (often used for empty lists)

    Comment(String),    // Comments
    Error(String)
}


struct Lexer {
    position: usize,
    input: Vec<u8>
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            position: 0,
            input: input.into_bytes()
        }
    }


    fn parse_symbol(&mut self, c: u8) -> Token {
        todo!()
    }
    fn parse_ident(&mut self) -> Token {
        Token::Ident("blah".to_string())
    }
    fn parse_number(&mut self) -> Token {
        todo!()
    }
    fn parse_comment(&mut self) -> Token {
        todo!()
    }
    fn parse_error(&mut self) -> Token {
        todo!()
    }

    fn peek(&self) -> Option<u8> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }
    fn advance(&mut self) {
        todo!()
    }
    fn read_char(&mut self) -> Option<u8> {
        let char = self.peek();
        self.advance();
        char
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek()  {
            if !(c.is_ascii_whitespace() || c == b',') {
                break;
            }
            self.advance();
        }
    }


}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let current_char = self.read_char();

        match current_char {
            Some(c) => {
                let token = match c {
                    b'(' => Token::LParen,
                    b')' => Token::RParen,
                    b'{' => Token::LSquiggly,
                    b'}' => Token::RSquiggly,
                    b'[' => Token::LSquare,
                    b']' => Token::RSquare,

                    b'+' => Token::Symbol("+".to_string()),
                    b'-' => Token::Symbol("-".to_string()),
                    b'*' => Token::Symbol("*".to_string()),
                    b'/' => Token::Symbol("/".to_string()),

                    b'~' => {
                        match self.peek() {
                            Some(b'@') => {self.advance();Token::Symbol("~@".to_string())},
                            _ => Token::Symbol("~".to_string()),
                        }
                    }

                    b'@' => Token::Symbol("@".to_string()),
                    b'\'' => Token::Symbol("'".to_string()),
                    b'`' => Token::Symbol("`".to_string()),
                    b'^' => Token::Symbol("^".to_string()),

                    b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.parse_ident(),

                    b'0'..=b'9' => self.parse_number(),

                    b';' => self.parse_comment(),

                    _ => Token::Error(c.to_string()),

                };
                Some(token)
            },
            None => None
        }
    }
}






























