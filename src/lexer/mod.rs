pub mod token;

#[cfg(test)]
mod tests;

use crate::lexer::token::Token;

/// Given the program represented as a string, lex it into tokens.
pub fn lex(program: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut token;
    let mut rest_program = program;

    loop {
        (token, rest_program) = next_token(rest_program);
        tokens.push(token);
        match tokens.last().unwrap() {
            Token::Eof => break,
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
    match chars.next() {
        Some(char) => match char {
            '=' => {
                if let Some(next_c) = chars.next()
                    && (next_c == '=')
                {
                    (Token::Eq, &program[2..])
                } else {
                    (Token::Assign, &program[1..])
                }
            }
            '+' => (Token::Plus, &program[1..]),
            '-' => (Token::Minus, &program[1..]),
            '!' => {
                if let Some(next_c) = chars.next()
                    && (next_c == '=')
                {
                    (Token::NotEq, &program[2..])
                } else {
                    (Token::Bang, &program[1..])
                }
            }
            '*' => (Token::Asterisk, &program[1..]),
            '/' => (Token::Slash, &program[1..]),
            '<' => (Token::Lt, &program[1..]),
            '>' => (Token::Gt, &program[1..]),
            ',' => (Token::Comma, &program[1..]),
            ';' => (Token::Semicolon, &program[1..]),
            '(' => (Token::LParen, &program[1..]),
            ')' => (Token::RParen, &program[1..]),
            '{' => (Token::LBrace, &program[1..]),
            '}' => (Token::RBrace, &program[1..]),
            'a'..='z' | 'A'..='Z' => read_ident(program),
            '0'..='9' => read_int(program),
            _ => (Token::Illegal(char), &program[1..]),
        },
        None => (Token::Eof, program),
    }
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
    &string[first_non_whitespace..]
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
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(String::from(string)),
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
    (lookup_ident(ident), rest)
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
    let val = int.parse::<usize>().unwrap();
    (Token::Int(val), rest)
}
