#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(char),
    Eof,

    // IDENTIFIERS AND LITERALS
    Ident(String),
    Int(usize),

    // OPERATORS
    Assign,   // "="
    Eq,       // "=="
    Plus,     // "+"
    Minus,    // "-"
    Bang,     // "!"
    NotEq,    // "!="
    Asterisk, // "*"
    Slash,    // "/"
    Lt,       // "<"
    Gt,       // ">"

    // DELIMITERS
    Comma,     // ","
    Semicolon, // ";"
    LParen,    // "("
    RParen,    // ")"
    LBrace,    // "{"
    RBrace,    // "}"

    // KEYWORDS
    Function, // "fn"
    Let,      // "let"
    True,     // "true"
    False,    // "false"
    If,       // "if"
    Else,     // "else"
    Return,   // "return"
}
