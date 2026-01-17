pub mod ast;

mod parse_exp;
mod parse_statement;

#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::Stmnt;
use crate::parser::parse_statement::parse_stmnt;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmnt> {
    let mut program = Vec::new();
    let mut next_statement;
    let mut rest_tokens = &tokens[0..];

    // read statements from our tokens until we reach EOF
    loop {
        (next_statement, rest_tokens) = parse_stmnt(rest_tokens);
        if next_statement == Stmnt::Illegal {
            rest_tokens = skip_errors(rest_tokens);
        }
        program.push(next_statement);
        match rest_tokens.first().unwrap() {
            Token::EoF => break, // unwrap is safe because EOF always exists
            _ => continue,
        }
    }

    program
}

// Skips all errors while parsing (fast forward until the next semicolon)
fn skip_errors(tokens: &[Token]) -> &[Token] {
    let mut next_semicolon = 0;
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Semicolon => {
                next_semicolon = index;
                break;
            }
            _ => continue,
        }
    }

    &tokens[next_semicolon..]
}
