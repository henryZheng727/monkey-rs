#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::{Exp, Precedence, UnaryOp};

pub(super) fn parse_exp(tokens: &[Token], prec: Precedence) -> (Exp, &[Token]) {
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

fn parse_prefix(tokens: &[Token]) -> (Exp, &[Token]) {
    // determine the operator
    let op = match tokens.first().unwrap() {
        Token::Bang => UnaryOp::Bang,
        Token::Minus => UnaryOp::Minus,
        _ => return (Exp::Illegal, tokens),
    };

    let (exp, rest_tokens) = parse_exp(&tokens[1..], Precedence::Prefix);
    match exp {
        Exp::Illegal => (Exp::Illegal, rest_tokens),
        _ => (Exp::PrefixOp(op, Box::new(exp)), rest_tokens),
    }
}

fn parse_infix(tokens: &[Token]) -> (Exp, &[Token]) {
    unimplemented!()
}
