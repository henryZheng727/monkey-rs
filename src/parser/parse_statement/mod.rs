#[cfg(test)]
mod tests;

use crate::lexer::token::Token;
use crate::parser::ast::{Exp, Prec, Stmnt};
use crate::parser::parse_exp::parse_exp;

/// Given a list of tokens, parse a statement from the start of the list.
/// We return the statement, and the rest of the unconsumed tokens.
///
/// A Monkey program is a list of statements.
/// A statement can take the form:
/// 1. let <id> = <exp>;
/// 2. return <exp>;
/// 3. <exp>
pub(super) fn parse_stmnt(tokens: &[Token]) -> (Stmnt, &[Token]) {
    match tokens.first().unwrap() {
        Token::Let => parse_let(&tokens[1..]),
        Token::Return => parse_return(&tokens[1..]),
        _ => parse_expression(tokens),
    }
}

/// Given a list of tokens (excluding LET token), we parse a LET Statement.
/// We return the statement, and the rest of the unconsumed tokens.
///
/// Note that since our statements are LL(1), we exclude the LET token.
/// The programmer must ensure they pass the correct slice of tokens.
/// e.g., if our input string was:
/// ```
/// let x = 5;
/// ```
///
/// We would feed into the parser:
/// ```
/// [Token::Ident(String::from("x"), Token::Eq, Token::Int(5),
///     Token::Semicolon, Token::EoF]
/// ```
/// And it would output
/// ```
/// (Stmnt::Let(String::from("x"), Exp::Int(5)),
///     [Token::EoF])
/// ```
fn parse_let(tokens: &[Token]) -> (Stmnt, &[Token]) {
    // expect an identifier
    let (id, tokens) = match tokens.first().unwrap() {
        Token::Ident(str) => (str, &tokens[1..]),
        _ => return (Stmnt::Illegal, tokens),
    };

    // expect an equals sign
    let tokens = match tokens.first().unwrap() {
        Token::Eq => &tokens[1..],
        _ => return (Stmnt::Illegal, tokens),
    };

    // parse an expression
    let (exp, tokens) = match parse_exp(tokens, Prec::Lowest) {
        (Exp::Illegal, rest_tokens) => return (Stmnt::Illegal, rest_tokens),
        ok => ok,
    };

    // expect a semicolon
    match tokens.first().unwrap() {
        Token::Semicolon => (Stmnt::Let(id.clone(), exp), &tokens[1..]),
        _ => (Stmnt::Illegal, tokens),
    }
}

/// Given a list of tokens (excluding RETURN token), parse a RETURN Statement.
/// We return the statement, and the rest of the unconsumed tokens.
///
/// Note that since our statements are LL(1), we exclude the RETURN token.
/// The programmer must ensure they pass the correct slice of tokens.
/// e.g., if our input string was:
/// ```
/// return 1000;
/// ```
///
/// We would feed into the parser:
/// ```
/// [Token::Int(1000), Token::Semicolon, Token::EoF]
/// ```
/// And it would output
/// ```
/// (Stmnt::Return(Exp::Int(1000)), [Token::EoF])
/// ```
fn parse_return(tokens: &[Token]) -> (Stmnt, &[Token]) {
    // expect an expression
    let (exp, tokens) = parse_exp(tokens, Prec::Lowest);
    let (return_stmnt, tokens) = match exp {
        Exp::Illegal => (Stmnt::Illegal, tokens),
        _ => (Stmnt::Return(exp), tokens),
    };

    // expect a semicolon
    match tokens.first().unwrap() {
        Token::Semicolon => (return_stmnt, &tokens[1..]),
        _ => (Stmnt::Illegal, tokens),
    }
}

/// Given a list of tokens, parse it as an expression.
/// We return the statement, and the rest of the unconsumed tokens.
fn parse_expression(tokens: &[Token]) -> (Stmnt, &[Token]) {
    let (exp, tokens) = parse_exp(tokens, Prec::Lowest);
    match exp {
        Exp::Illegal => (Stmnt::Illegal, tokens),
        _ => (Stmnt::Exp(exp), tokens),
    }
}
