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
    Symbol(String),     // For any special symbol or combinations like +, -, =, >=, ~@, etc
    Ident(String),      // Identifiers such as function names
    Keyword(String),    // Keywords

    // Booleans and Nil
    True,               // Boolean true
    False,              // Boolean false
    Nil,                // Nil (often used for empty lists)

    Comment(String),    // Comments
}