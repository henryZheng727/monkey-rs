use crate::lexer::token::Token;
use crate::parser::ast::Exp;

pub(super) fn parse_exp(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_ident(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_int(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_bool(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_prefix_op(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_infix_op(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}
