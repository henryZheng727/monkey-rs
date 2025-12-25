pub enum Token {
    ILLEGAL,
    EOF,

    // IDENTIFIERS AND LITERALS
    IDENT(String),
    INT(usize),

    // OPERATORS
    ASSIGN,
    PLUS,

    // DELIMITERS
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // KEYWORDS
    FUNCTION,
    LET,
}

pub fn lex(program: &String) -> Vec<Token> {
    unimplemented!()
}
