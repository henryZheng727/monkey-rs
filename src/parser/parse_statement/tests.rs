use super::*;
use crate::lexer::token::Token;
use crate::parser::ast::Exp;

#[test]
fn test_parse_let_small() {
    let test_tokens = vec![
        Token::Let,
        Token::Ident(String::from("x")),
        Token::Assign,
        Token::Int(5),
    ];
    let expected = Stmnt::Let(String::from("x"), Exp::Int(5));
    let (actual, _) = parse_let(&test_tokens);
    assert_eq!(expected, actual)
}
