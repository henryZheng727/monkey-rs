#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL(char),
    EOF,

    // IDENTIFIERS AND LITERALS
    IDENT(String),
    INT(usize),

    // OPERATORS
    ASSIGN,   // "="
    EQ,       // "=="
    PLUS,     // "+"
    MINUS,    // "-"
    BANG,     // "!"
    NOTEQ,    // "!="
    ASTERISK, // "*"
    SLASH,    // "/"
    LT,       // "<"
    GT,       // ">"

    // DELIMITERS
    COMMA,     // ","
    SEMICOLON, // ";"
    LPAREN,    // "("
    RPAREN,    // ")"
    LBRACE,    // "{"
    RBRACE,    // "}"

    // KEYWORDS
    FUNCTION, // "fn"
    LET,      // "let"
    TRUE,     // "true"
    FALSE,    // "false"
    IF,       // "if"
    ELSE,     // "else"
    RETURN,   // "return"
}
