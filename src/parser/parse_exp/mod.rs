#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::Exp;

pub(super) fn parse_exp(tokens: &[Token], prec: u8) -> (Exp, &[Token]) {
    unimplemented!()
}

fn parse_ident(tokens: &[Token], prec: u8) -> (Exp, &[Token]) {
    unimplemented!()
}

fn parse_int(tokens: &[Token], prec: u8) -> (Exp, &[Token]) {
    unimplemented!()
}

fn parse_bool(tokens: &[Token], prec: u8) -> (Exp, &[Token]) {
    unimplemented!()
}

fn parse_prefix_op(tokens: &[Token], prec: u8) -> (Exp, &[Token]) {
    unimplemented!()
}

fn parse_infix_op(tokens: &[Token], prec: u8) -> (Exp, &[Token]) {
    unimplemented!()
}
