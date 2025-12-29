pub mod ast;

#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::Exp;
use crate::parser::ast::Stmnt;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmnt> {
    unimplemented!()
}

fn parse_stmnt(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_stmnt_let(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_stmnt_return(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_stmnt_expression(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
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
