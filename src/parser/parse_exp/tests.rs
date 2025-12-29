use super::*;

use crate::lexer::token::Token;

#[test]
fn test_parse_int() {
    let test_tokens = vec![Token::INT(5)];
    let expected = Exp::INT(5);
    let (actual, _) = parse_int(&test_tokens, 0);
    assert_eq!(expected, actual);
}
