#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::Exp;
use crate::parser::ast::Stmnt;
use crate::parser::parse_exp::parse_exp;

pub(super) fn parse_stmnt(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    match tokens.first() {
        Some(token) => match token {
            Token::LET => parse_let(tokens),
            Token::RETURN => parse_return(tokens),
            _ => parse_expression(tokens),
        },
        None => unreachable!(), // there will always be EOF
    }
}

fn parse_let(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_return(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_expression(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    let (exp, rest) = parse_exp(tokens, 0);
    return (Stmnt::EXPRESSION(exp), rest);
}
