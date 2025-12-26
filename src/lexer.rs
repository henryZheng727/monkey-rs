#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL(char),
    EOF,

    // IDENTIFIERS AND LITERALS
    IDENT(String),
    INT(usize),

    // OPERATORS
    ASSIGN,   // "="
    PLUS,     // "+"
    MINUS,    // "-"
    BANG,     // "!"
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
pub fn next_token(program: &str) -> (Token, &str) {
    // Cut leading whitespace.
    let program = eat_whitespace(program);

    // Determine what the first token is and lex it.
    match program.chars().next() {
        Some(char) => {
            return match char {
                '=' => (Token::ASSIGN, &program[1..]),
                '+' => (Token::PLUS, &program[1..]),
                '-' => (Token::MINUS, &program[1..]),
                '!' => (Token::BANG, &program[1..]),
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
            };
        }
        None => return (Token::EOF, program),
    }
}

/// Cuts all whitespace from the start of a string.
pub fn eat_whitespace(string: &str) -> &str {
    for (index, character) in string.chars().enumerate() {
        if !character.is_whitespace() {
            return &string[index..];
        }
    }
    return "";
}

pub fn read_ident(string: &str) -> (Token, &str) {
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

    for (index, character) in string.chars().enumerate() {
        if !is_letter(character) {
            return (lookup_ident(&string[..index]), &string[index..]);
        }
    }

    return (lookup_ident(string), "");
}

pub fn read_int(string: &str) -> (Token, &str) {
    for (index, character) in string.chars().enumerate() {
        if !character.is_numeric() {
            let val = match usize::from_str_radix(&string[..index], 10) {
                Ok(value) => value,
                Err(_) => unreachable!(),
            };
            return (Token::INT(val), &string[index..]);
        }
    }

    let val = match usize::from_str_radix(&string, 10) {
        Ok(value) => value,
        Err(_) => unreachable!(),
    };
    return (Token::INT(val), "");
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
    fn test_small_with_whitespace() {
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
    fn test_medium() {
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
    fn test_illegal_small() {
        let test_string = "~";
        let expected = vec![Token::ILLEGAL('~'), Token::EOF];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_illegal_with_whitespace() {
        let test_string = "  ~   ~    ";
        let expected = vec![Token::ILLEGAL('~'), Token::ILLEGAL('~'), Token::EOF];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_empty_string() {
        let test_string = "";
        let expected = vec![Token::EOF];
        assert_eq!(expected, lex(test_string));
    }

    #[test]
    fn test_large_no_multichar_ops() {
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
}
