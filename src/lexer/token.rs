#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(char),
    EoF,

    // IDENTIFIERS AND LITERALS
    Ident(String),
    Bool(bool),
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
    If,       // "if"
    Else,     // "else"
    Return,   // "return"
}
