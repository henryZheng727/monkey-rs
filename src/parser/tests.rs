use crate::lexer::token::Token;
use crate::parser::ast::Exp;
use crate::parser::ast::Stmnt;
use crate::parser::parse;

#[test]
fn test_parse_small() {
    let test_tokens = vec![
        Token::Let,
        Token::Ident(String::from("x")),
        Token::Assign,
        Token::Int(5),
    ];
    let expected = vec![Stmnt::Let(String::from("x"), Exp::Int(5))];
    let actual = parse(test_tokens);
    assert_eq!(expected, actual)
}
