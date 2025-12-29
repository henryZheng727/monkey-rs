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
    let mut next_statement: Stmnt;
    let mut rest_tokens: &Vec<Token> = &tokens;

    // read statements from our tokens until we reach EOF
    loop {
        (next_statement, rest_tokens) = parse_stmnt(rest_tokens);
        program.push(next_statement);
        match rest_tokens.first().unwrap() {
            Token::EOF => break, // unwrap is safe because EOF always exists
            _ => continue,
        }
    }

    program
}
