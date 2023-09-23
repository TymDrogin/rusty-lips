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

    fn next_token(&mut self) -> Token {

        let token = match self.peek() {
            Some(c) => match c {
                    b'(' => Token::LParen,
                    b')' => Token::RParen,
                    b'{' => Token::LSquiggly,
                    b'}' => Token::RSquiggly,
                    b'[' => Token::LSquare,
                    b']' => Token::RSquare,

                    b'\'' => Token::Symbol("'".to_string()),
                    b'`' => Token::Symbol("`".to_string()),
                    b'^' => Token::Symbol("^".to_string()),
                    b'@' => Token::Symbol("@".to_string()),
                    b'~' => {
                        self.read_char();
                        match self.peek() {
                            Some(b'@') => Token::Symbol("~@".to_string()),
                            _ => Token::Symbol("~".to_string()),
                        }
                    },

                    b'"' => self.parse_string(),
                    b';' => self.parse_comment(),

                    b'0'..=b'9' => self.parse_number(),
                    b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.parse_ident(),

            }
            None => {
                Token::EoF
            }
        };
    }




    fn parse_number(&mut self) -> Token{
        let mut number_str = String::new();

        while let Some(curr_char) = self.peek() {
            if curr_char.is_ascii_digit() || curr_char == b'.' {
                number_str.push(curr_char as char);
                self.read_char();
            } else {
                break; // Exit the loop on non-digit or non-dot character
            }
        }
        // Attempt to parse as an integer first
        if let Ok(int_value) = number_str.parse::<i64>() {
            Token::Integer(int_value);
        }

        // If parsing as an integer fails, try parsing as a float
        if let Ok(float_value) = number_str.parse::<f64>() {
            Token::Float(float_value);
        }
        panic!("Number parsing fucked-up")
    }
    fn parse_ident(&mut self) -> Token {
        while let Some(curr_char) = self.peek() {
            todo!();
        }







        return Token::EoF
    }
    fn parse_string(&mut self) -> Token {
        todo!()
    }
    fn parse_comment(&mut self) -> Token {
        todo!();
    }
    fn read_char(&mut self) {
        self.curr_char = self.peek();
        self.advance();
    }
    fn peek(&self) -> Option<u8> {
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
            self.read_char();
        }
    }
}