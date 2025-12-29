use super::*;
use crate::lexer::token::Token;

#[test]
fn test_parse_let_small() {
    let test_tokens = vec![
        Token::LET,
        Token::IDENT(String::from("x")),
        Token::ASSIGN,
        Token::INT(5),
    ];
    let expected = Stmnt::LET(String::from("x"), Exp::INT(5));
    let (actual, _) = parse_let(&test_tokens);
    assert_eq!(expected, actual)
}
