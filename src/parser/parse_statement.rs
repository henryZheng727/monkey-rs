use crate::lexer::token::Token;
use crate::parser::ast::Exp;
use crate::parser::ast::Stmnt;

pub(super) fn parse_stmnt(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    match tokens.first() {
        Some(token) => match token {
            Token::LET => parse_stmnt_let(tokens),
            Token::RETURN => parse_stmnt_return(tokens),
            _ => parse_stmnt_expression(tokens),
        },
        None => unreachable!(), // there will always be EOF
    }
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
