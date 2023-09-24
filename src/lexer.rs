use logos::{Logos, Lexer};

fn to_string(lex: &mut Lexer<Token>) -> Option<String>{
    Some(lex.slice().to_string())
}


#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\s,]")] // Ignore this regex pattern between tokens
pub enum Token {
    // Parentheses and Brackets

    #[token("(")]
    LParen,             // Left parenthesis
    #[token(")")]
    RParen,             // Right parenthesis
    #[token("{")]
    LSquiggly,          // Left brace
    #[token("}")]
    RSquiggly,          // Right brace
    #[token("[")]
    LSquare,            // Left square bracket
    #[token("]")]
    RSquare,            // Right square bracket

    #[regex(r"-?[0-9]*(.[0-9]+)?", to_string)]
    Number(String),
    #[regex(r#"(?:[^"\\]|\\.)*"#, to_string)]
    String(String),     // String literals

    //Symbols, ident and keyword
    //FIXME
    #[token("+", "-", "*", "/"
    "'", "`", "^", "~@", "~", "@")
    ]
    Symbol(String),     // For any special symbol or combinations like +, -, =, >=, ~@, etc
    #[regex("[a-zA-Z_]*", to_string)]
    Ident(String),      // Identifiers such as function names
    //FIXME
    #[token("defun",to_string)]
    Keyword(String),    // Keywords

    // Booleans and Nil
    #[token("true")]
    True,               // Boolean true
    #[token("false")]
    False,              // Boolean false
    #[token("nil")]
    Nil,                // Nil (often used for empty lists)
    #[regex(";.*", to_string)]
    Comment(String),    // Comments
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_recognize_keywords() {
        let mut lexer = Token::lexer("true false nil defun");

        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::False));
        assert_eq!(lexer.next(), Some(Token::Nil));
        assert_eq!(lexer.next(), Some(Token::Keyword("defun".to_owned())));
    }
}