#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::Exp;

pub(super) fn parse_exp(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_ident(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_int(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_bool(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_prefix_op(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_infix_op(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}
