use super::*;

use crate::lexer::token::Token;

#[test]
fn test_parse_int() {
    let test_tokens = vec![Token::Int(5)];
    let expected = Exp::Int(5);
    let (actual, _) = parse_int(&test_tokens, 0);
    assert_eq!(expected, actual);
}
