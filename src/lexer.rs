#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL(char),
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

pub fn lex(program: &str) -> Vec<Token> {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_small() {
        let test_string = "=+(){},;";
        let expected = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_large() {
        let test_string = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "#;
        let expected = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_illegal_small() {
        let test_string = "~";
        let expected = vec![Token::ILLEGAL('~'), Token::EOF];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_empty_string() {
        let test_string = "=+(){},;";
        let expected = vec![Token::EOF];
        assert_eq!(expected, lex(test_string));
    }
}
