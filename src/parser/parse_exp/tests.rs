use super::*;

use crate::lexer::token::Token;

#[test]
fn test_parse_ident() {
    let test_tokens = vec![Token::Ident(String::from("x"))];
    let expected = Exp::Ident(String::from("x"));
    let (actual, _) = parse_ident(&test_tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_ident_illegal() {
    let test_tokens = vec![Token::Bool(true)];
    let expected = Exp::Illegal;
    let (actual, _) = parse_ident(&test_tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_int() {
    let test_tokens = vec![Token::Int(5)];
    let expected = Exp::Int(5);
    let (actual, _) = parse_int(&test_tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_int_zero() {
    let test_tokens = vec![Token::Int(0)];
    let expected = Exp::Int(0);
    let (actual, _) = parse_int(&test_tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_int_illegal() {
    let test_tokens = vec![Token::Bool(true)];
    let expected = Exp::Illegal;
    let (actual, _) = parse_int(&test_tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_bool_true() {
    let test_tokens = vec![Token::Bool(true)];
    let expected = Exp::Bool(true);
    let (actual, _) = parse_bool(&test_tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_bool_false() {
    let test_tokens = vec![Token::Bool(false)];
    let expected = Exp::Bool(false);
    let (actual, _) = parse_bool(&test_tokens);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_bool_illegal() {
    let test_tokens = vec![Token::Int(5)];
    let expected = Exp::Illegal;
    let (actual, _) = parse_bool(&test_tokens);
    assert_eq!(expected, actual);
}
