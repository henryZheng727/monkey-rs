use crate::lexer::token::Token;
use crate::parser::ast::Exp;
use crate::parser::ast::Stmnt;
use crate::parser::parse;

#[test]
fn test_parse_small() {
    let test_tokens = vec![
        Token::LET,
        Token::IDENT(String::from("x")),
        Token::ASSIGN,
        Token::INT(5),
    ];
    let expected = vec![Stmnt::LET(String::from("x"), Exp::INT(5))];
    let actual = parse(test_tokens);
    assert_eq!(expected, actual)
}
