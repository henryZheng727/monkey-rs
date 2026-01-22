use super::*;
use crate::lexer::token::Token;
use crate::parser::ast::Exp;

#[test]
fn test_parse_stmnt_let_small() {
    let test_tokens = vec![
        Token::Let,
        Token::Ident(String::from("x")),
        Token::Assign,
        Token::Int(5),
        Token::EoF,
    ];
    let expected_stmnt = Stmnt::Let(String::from("x"), Exp::Int(5));
    let expected_rest = vec![Token::EoF];
    let (stmnt, rest) = parse_stmnt(&test_tokens);
    assert_eq!(expected_stmnt, stmnt);
    assert_eq!(expected_rest, rest)
}

#[test]
fn test_parse_stmnt_return_small() {
    let test_tokens =
        vec![Token::Return, Token::Int(5), Token::Semicolon, Token::EoF];
    let expected_stmnt = Stmnt::Return(Exp::Int(5));
    let expected_rest = vec![Token::EoF];
    let (stmnt, rest) = parse_stmnt(&test_tokens);
    assert_eq!(expected_stmnt, stmnt);
    assert_eq!(expected_rest, rest)
}

#[test]
fn test_parse_stmnt_expression_small() {
    let test_tokens = vec![Token::Int(5), Token::EoF];
    let expected_stmnt = Stmnt::Return(Exp::Int(5));
    let expected_rest = vec![Token::EoF];
    let (stmnt, rest) = parse_stmnt(&test_tokens);
    assert_eq!(expected_stmnt, stmnt);
    assert_eq!(expected_rest, rest)
}

#[test]
fn test_parse_let_small() {
    let test_tokens = vec![
        Token::Ident(String::from("x")),
        Token::Assign,
        Token::Int(5),
        Token::EoF,
    ];
    let expected_stmnt = Stmnt::Let(String::from("x"), Exp::Int(5));
    let expected_rest = vec![Token::EoF];
    let (stmnt, rest) = parse_let(&test_tokens);
    assert_eq!(expected_stmnt, stmnt);
    assert_eq!(expected_rest, rest)
}

#[test]
fn test_parse_return_small() {
    let test_tokens = vec![Token::Int(5), Token::EoF];
    let expected_stmnt = Stmnt::Return(Exp::Int(5));
    let expected_rest = vec![Token::EoF];
    let (stmnt, rest) = parse_return(&test_tokens);
    assert_eq!(expected_stmnt, stmnt);
    assert_eq!(expected_rest, rest)
}

#[test]
fn test_parse_expression_small() {
    let test_tokens = vec![Token::Int(5), Token::EoF];
    let expected_stmnt = Stmnt::Return(Exp::Int(5));
    let expected_rest = vec![Token::EoF];
    let (stmnt, rest) = parse_expression(&test_tokens);
    assert_eq!(expected_stmnt, stmnt);
    assert_eq!(expected_rest, rest)
}
