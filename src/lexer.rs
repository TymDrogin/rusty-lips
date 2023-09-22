use anyhow::Result;
#[derive(Debug, PartialEq)]
enum Token {
    EoF,          // End of input

    // Parentheses and Brackets
    LParen,    // Left parenthesis
    RParen,    // Right parenthesis
    LSquiggly, // Left brace
    RSquiggly, // Right brace
    LSquare,   // Left square bracket
    RSquare,   // Right square bracket

    // Literals
    Integer(i64), // Integer literals
    Float(f64),   // Floating-point literals
    String(String),// String literals
    Symbol(String),// Symbols and Identifiers
    Keyword(String),// Keywords

    // Special Characters
    Quote,         // Quote character (')
    Dot,           // Dot (.) for cons cells

    // Booleans and Nil
    True,          // Boolean true
    False,         // Boolean false
    Nil,           // Nil (often used for empty lists)

    Comment(String), // Comments
}

#[allow(unused)]
pub struct Lexer {
    position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        return lex;
    }


    fn next(&mut self) -> Result<Token> {
        self.skip_whitespace();
        
        let token = match self.ch {
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LSquiggly,
            b'}' => Token::RSquiggly,
            b'[' => Token::LSquare,
            b']' => Token::RSquare,

            b';' => {
                let mut comment = String::new();
                loop {
                    self.read_char();
                    if self.peek() == b'\n' {
                        break;
                    }
                    comment.push(str::from_utf8(self.ch));
                }
                Token::Comment(comment)
            }






            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {}
            b'0'..=b'9' => {}

            0 => Token::EoF,
        };
    }


    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.position];
        }
        self.advance();
    }
    fn read_identifier() {
        TODO!();
    }
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() || self.ch == b',' {
        self.read_char();
        }
    }
    fn advance(&mut self) {self.position += 1;}
    fn peek(&self) -> u8 {
        if self.position < self.input.len() {
            return self.input[self.ch];
        } else {
            return 0;
        }
    }

}
