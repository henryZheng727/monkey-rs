#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::Exp;
use crate::parser::ast::Stmnt;
use crate::parser::parse_exp::parse_exp;

pub(super) fn parse_stmnt(tokens: &[Token]) -> (Stmnt, &[Token]) {
    match tokens.first().unwrap() {
        Token::LET => parse_let(tokens),
        Token::RETURN => parse_return(tokens),
        _ => parse_expression(tokens),
    }
}

fn parse_let(tokens: &[Token]) -> (Stmnt, &[Token]) {
    unimplemented!()
}

fn parse_return(tokens: &[Token]) -> (Stmnt, &[Token]) {
    unimplemented!()
}

fn parse_expression(tokens: &[Token]) -> (Stmnt, &[Token]) {
    let (exp, rest) = parse_exp(tokens, 0);
    return (Stmnt::EXPRESSION(exp), rest);
}
