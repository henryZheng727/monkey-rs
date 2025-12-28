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

/// Given the program represented as a string, lex it into tokens.
pub fn lex(program: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut curr_string: &str = &program;

    loop {
        let (next_token, next_string) = next_token(curr_string);
        match next_token {
            Token::EOF => {
                tokens.push(next_token);
                break;
            }
            _ => tokens.push(next_token),
        }
        curr_string = next_string;
    }

    tokens
}

/// Given the program represented as a string, get the next token from it.
fn next_token(program: &str) -> (Token, &str) {
    // cut leading whitespace
    let program = eat_whitespace(program);

    // determine what the first token is and lex it
    let mut chars = program.chars();
    return match chars.next() {
        Some(char) => match char {
            '=' => {
                if let Some(next_c) = chars.next()
                    && (next_c == '=')
                {
                    (Token::EQ, &program[2..])
                } else {
                    (Token::ASSIGN, &program[1..])
                }
            }
            '+' => (Token::PLUS, &program[1..]),
            '-' => (Token::MINUS, &program[1..]),
            '!' => {
                if let Some(next_c) = chars.next()
                    && (next_c == '=')
                {
                    (Token::NOTEQ, &program[2..])
                } else {
                    (Token::BANG, &program[1..])
                }
            }
            '*' => (Token::ASTERISK, &program[1..]),
            '/' => (Token::SLASH, &program[1..]),
            '<' => (Token::LT, &program[1..]),
            '>' => (Token::GT, &program[1..]),
            ',' => (Token::COMMA, &program[1..]),
            ';' => (Token::SEMICOLON, &program[1..]),
            '(' => (Token::LPAREN, &program[1..]),
            ')' => (Token::RPAREN, &program[1..]),
            '{' => (Token::LBRACE, &program[1..]),
            '}' => (Token::RBRACE, &program[1..]),
            'a'..='z' | 'A'..='Z' => read_ident(program),
            '0'..='9' => read_int(program),
            _ => (Token::ILLEGAL(char), &program[1..]),
        },
        None => (Token::EOF, program),
    };
}

/// Cuts all whitespace from the start of a string.
fn eat_whitespace(string: &str) -> &str {
    // collect all whitespace at start of string
    let mut first_non_whitespace = string.len();
    for (index, char) in string.char_indices() {
        if !char.is_whitespace() {
            first_non_whitespace = index;
            break;
        }
    }

    // slice the leading whitespace
    return &string[first_non_whitespace..];
}

fn read_ident(string: &str) -> (Token, &str) {
    // an identifier is [a-zA-Z_]+
    fn is_letter(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    // determine if an identifier is a keyword
    fn lookup_ident(string: &str) -> Token {
        match string {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            _ => Token::IDENT(String::from(string)),
        }
    }

    // collect all letters at start of string
    let mut first_non_letter = string.len();
    for (index, char) in string.char_indices() {
        if !is_letter(char) {
            first_non_letter = index;
            break;
        }
    }

    // construct a token
    let ident = &string[..first_non_letter];
    let rest = &string[first_non_letter..];
    return (lookup_ident(ident), rest);
}

fn read_int(string: &str) -> (Token, &str) {
    // collect all integer characters at start of string
    let mut first_non_int = string.len();
    for (index, char) in string.char_indices() {
        if !char.is_numeric() {
            first_non_int = index;
            break;
        }
    }

    // construct a token
    let int = &string[..first_non_int];
    let rest = &string[first_non_int..];
    let val = usize::from_str_radix(int, 10).unwrap();
    return (Token::INT(val), rest);
}

#[cfg(test)]
mod tests {

    use super::*;

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
