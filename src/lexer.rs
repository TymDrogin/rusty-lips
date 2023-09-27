
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
    Number(String),     // Number stored as string
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

#[derive(Debug, PartialEq)]
pub struct Lexer {
    position: usize,
    input: Vec<u8>
}

#[allow(unused)]
impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            position: 0,
            input: input.into_bytes()
        }
    }


    fn parse_ident(&mut self, c: u8) -> Token {
        let mut ident = String::new();
        ident.push(c as char);

        while let Some(c) = self.read_char() {
            if !(c.is_ascii_alphabetic() || c == b'_') {
                break;
            }
            ident.push(c as char);
        }

        match ident.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "nil" => Token::Nil,
            _ => Token::Ident(ident),
        }
    }
    fn parse_number(&mut self, c: u8) -> Token {
        let mut number = String::new();
        number.push(c as char);

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                number.push(c as char);
                self.advance();
            } else if c == b'.' {
                // Handle floating-point numbers
                number.push(c as char);
                self.advance(); // Consume the period

                // Continue parsing digits after the period
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        number.push(c as char);
                        self.advance();
                    } else {
                        break; // Exit the loop if a non-digit is encountered
                    }
                }
            } else {
                break; // Exit the loop if a non-digit and non-period character is encountered
            }
        }
        Token::Number(number)
    }

    fn parse_comment(&mut self) -> Token {
        let mut comment = String::new();

        while let Some(c) = self.peek() {
            if c == b'\n' || c == 0 {
                break;
            }
            comment.push(c as char);
            self.advance();
        }
        Token::Comment(comment)
    }

    fn peek(&self) -> Option<u8> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn read_char(&mut self) -> Option<u8> {
        let char = self.peek();
        self.advance();
        char
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !(c.is_ascii_whitespace() || c == b',') {
                break;
            } else {
                self.advance();
            }
        }
    }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let tok = match self.read_char() {
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
                        if let Some(b'@') = self.peek() {
                            self.advance();
                            Token::Symbol("~@".to_string())
                        } else {
                            Token::Symbol("~".to_string())
                        }
                    }

                    b'@' => Token::Symbol("@".to_string()),
                    b'\'' => Token::Symbol("'".to_string()),
                    b'`' => Token::Symbol("`".to_string()),
                    b'^' => Token::Symbol("^".to_string()),

                    b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.parse_ident(c),

                    b'0'..=b'9' => self.parse_number(c),

                    b';' => self.parse_comment(),

                    _ => Token::Error(c.to_string())
                };
                self.advance();
                Some(token)
            }
            None => None
        };
        tok
    }
}

