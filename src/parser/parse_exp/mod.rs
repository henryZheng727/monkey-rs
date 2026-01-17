#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::Exp;

pub(super) fn parse_exp(tokens: &[Token], prec: u8) -> (Exp, &[Token]) {
    unimplemented!()
}

fn parse_ident(tokens: &[Token]) -> (Exp, &[Token]) {
    match tokens.first().unwrap() {
        Token::Ident(str) => (Exp::Ident(str.clone()), &tokens[1..]),
        _ => (Exp::Illegal, tokens),
    }
}

fn parse_int(tokens: &[Token]) -> (Exp, &[Token]) {
    match tokens.first().unwrap() {
        Token::Int(num) => (Exp::Int(*num), &tokens[1..]),
        _ => (Exp::Illegal, tokens),
    }
}

fn parse_bool(tokens: &[Token]) -> (Exp, &[Token]) {
    match tokens.first().unwrap() {
        Token::Bool(val) => (Exp::Bool(*val), &tokens[1..]),
        _ => (Exp::Illegal, tokens),
    }
}

fn parse_prefix_op(tokens: &[Token]) -> (Exp, &[Token]) {
    unimplemented!()
}

fn parse_infix_op(tokens: &[Token]) -> (Exp, &[Token]) {
    unimplemented!()
}
