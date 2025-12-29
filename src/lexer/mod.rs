pub mod token;

#[cfg(test)]
mod tests;

use crate::lexer::token::Token;

/// Given the program represented as a string, lex it into tokens.
pub fn lex(program: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut token: Token;
    let mut rest_program: &str = &program;

    loop {
        (token, rest_program) = next_token(rest_program);
        tokens.push(token);
        match tokens.last().unwrap() {
            Token::EOF => break,
            _ => continue,
        }
    }

    tokens
}

/// Given the program represented as a string, get the next token from it.
/// Return the rest of the program (which has not been lexed).
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

/// Read an identifier from the start of the string.
/// Return the rest of the string.
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

/// Read an integer from the start of the string.
/// Return the rest of the string.
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
