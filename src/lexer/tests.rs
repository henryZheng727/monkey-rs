#[cfg(test)]
mod tests {

    use crate::lexer::*;

    #[test]
    fn test_lex_small() {
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
    fn test_lex_small_with_whitespace() {
        let test_string = "        =+(){},;       ";
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
    fn test_lex_medium() {
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
            Token::IDENT(String::from("result")),
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
    fn test_lex_illegal_small() {
        let test_string = "~";
        let expected = vec![Token::ILLEGAL('~'), Token::EOF];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_lex_illegal_with_whitespace() {
        let test_string = "  ~   ~    ";
        let expected = vec![Token::ILLEGAL('~'), Token::ILLEGAL('~'), Token::EOF];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_lex_empty_string() {
        let test_string = "";
        let expected = vec![Token::EOF];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_lex_large_no_multichar_ops() {
        let test_string = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
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
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::EOF,
        ];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_lex_large() {
        let test_string = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10;
            10 != 9;
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
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NOTEQ,
            Token::INT(9),
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_eat_whitespace_small() {
        let test_string = "     test    ";
        let expected = "test    ";
        assert_eq!(expected, eat_whitespace(test_string));
    }

    #[test]
    fn test_eat_whitespace_empty_string() {
        let test_string = "";
        let expected = "";
        assert_eq!(expected, eat_whitespace(test_string));
    }

    #[test]
    fn test_eat_whitespace_no_whitespace() {
        let test_string = "nowhitespace";
        let expected = "nowhitespace";
        assert_eq!(expected, eat_whitespace(test_string));
    }

    #[test]
    fn test_eat_whitespace_newlines() {
        let test_string = "\n\n\n\n\ntest";
        let expected = "test";
        assert_eq!(expected, eat_whitespace(test_string));
    }

    #[test]
    fn test_read_ident_small() {
        let test_string = "test the following";
        let expected = (Token::IDENT(String::from("test")), " the following");
        assert_eq!(expected, read_ident(test_string));
    }

    #[test]
    fn test_read_ident_one_char() {
        let test_string = "a b c";
        let expected = (Token::IDENT(String::from("a")), " b c");
        assert_eq!(expected, read_ident(test_string));
    }

    #[test]
    fn test_read_ident_full_string() {
        let test_string = "tokenisfullstring";
        let expected = (Token::IDENT(String::from("tokenisfullstring")), "");
        assert_eq!(expected, read_ident(test_string));
    }

    #[test]
    fn test_read_int_small() {
        let test_string = "1234 test";
        let expected = (Token::INT(1234), " test");
        assert_eq!(expected, read_int(test_string));
    }

    #[test]
    fn test_read_int_one_char() {
        let test_string = "1 2 3";
        let expected = (Token::INT(1), " 2 3");
        assert_eq!(expected, read_int(test_string));
    }

    #[test]
    fn test_read_int_full_string() {
        let test_string = "1234";
        let expected = (Token::INT(1234), "");
        assert_eq!(expected, read_int(test_string));
    }
}
