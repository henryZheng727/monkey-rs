use crate::lexer::*;

#[test]
fn test_lex_small() {
    let test_string = "=+(){},;";
    let expected = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Semicolon,
        Token::Eof,
    ];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_lex_small_with_whitespace() {
    let test_string = "        =+(){},;       ";
    let expected = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Semicolon,
        Token::Eof,
    ];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_lex_medium() {
    let test_string = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "#;
    let expected = vec![
        Token::Let,
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident(String::from("x")),
        Token::Comma,
        Token::Ident(String::from("y")),
        Token::RParen,
        Token::LBrace,
        Token::Ident(String::from("x")),
        Token::Plus,
        Token::Ident(String::from("y")),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("result")),
        Token::Assign,
        Token::Ident(String::from("add")),
        Token::LParen,
        Token::Ident(String::from("five")),
        Token::Comma,
        Token::Ident(String::from("ten")),
        Token::RParen,
        Token::Semicolon,
        Token::Eof,
    ];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_lex_illegal_small() {
    let test_string = "~";
    let expected = vec![Token::Illegal('~'), Token::Eof];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_lex_illegal_with_whitespace() {
    let test_string = "  ~   ~    ";
    let expected = vec![Token::Illegal('~'), Token::Illegal('~'), Token::Eof];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_lex_empty_string() {
    let test_string = "";
    let expected = vec![Token::Eof];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_lex_large_no_multichar_ops() {
    let test_string = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
        "#;
    let expected = vec![
        Token::Let,
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident(String::from("x")),
        Token::Comma,
        Token::Ident(String::from("y")),
        Token::RParen,
        Token::LBrace,
        Token::Ident(String::from("x")),
        Token::Plus,
        Token::Ident(String::from("y")),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("result")),
        Token::Assign,
        Token::Ident(String::from("add")),
        Token::LParen,
        Token::Ident(String::from("five")),
        Token::Comma,
        Token::Ident(String::from("ten")),
        Token::RParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int(5),
        Token::Semicolon,
        Token::Int(5),
        Token::Lt,
        Token::Int(10),
        Token::Gt,
        Token::Int(5),
        Token::Semicolon,
        Token::If,
        Token::LParen,
        Token::Int(5),
        Token::Lt,
        Token::Int(10),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RBrace,
        Token::Eof,
    ];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_lex_large() {
    let test_string = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10;
            10 != 9;
        "#;
    let expected = vec![
        Token::Let,
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident(String::from("x")),
        Token::Comma,
        Token::Ident(String::from("y")),
        Token::RParen,
        Token::LBrace,
        Token::Ident(String::from("x")),
        Token::Plus,
        Token::Ident(String::from("y")),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident(String::from("result")),
        Token::Assign,
        Token::Ident(String::from("add")),
        Token::LParen,
        Token::Ident(String::from("five")),
        Token::Comma,
        Token::Ident(String::from("ten")),
        Token::RParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int(5),
        Token::Semicolon,
        Token::Int(5),
        Token::Lt,
        Token::Int(10),
        Token::Gt,
        Token::Int(5),
        Token::Semicolon,
        Token::If,
        Token::LParen,
        Token::Int(5),
        Token::Lt,
        Token::Int(10),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RBrace,
        Token::Int(10),
        Token::Eq,
        Token::Int(10),
        Token::Semicolon,
        Token::Int(10),
        Token::NotEq,
        Token::Int(9),
        Token::Semicolon,
        Token::Eof,
    ];
    assert_eq!(expected, lex(test_string));
}

#[test]
fn test_eat_whitespace_small() {
    let test_string = "     test    ";
    let expected = "test    ";
    assert_eq!(expected, eat_whitespace(test_string));
}

#[test]
fn test_eat_whitespace_empty_string() {
    let test_string = "";
    let expected = "";
    assert_eq!(expected, eat_whitespace(test_string));
}

#[test]
fn test_eat_whitespace_no_whitespace() {
    let test_string = "nowhitespace";
    let expected = "nowhitespace";
    assert_eq!(expected, eat_whitespace(test_string));
}

#[test]
fn test_eat_whitespace_newlines() {
    let test_string = "\n\n\n\n\ntest";
    let expected = "test";
    assert_eq!(expected, eat_whitespace(test_string));
}

#[test]
fn test_read_ident_small() {
    let test_string = "test the following";
    let expected = (Token::Ident(String::from("test")), " the following");
    assert_eq!(expected, read_ident(test_string));
}

#[test]
fn test_read_ident_one_char() {
    let test_string = "a b c";
    let expected = (Token::Ident(String::from("a")), " b c");
    assert_eq!(expected, read_ident(test_string));
}

#[test]
fn test_read_ident_full_string() {
    let test_string = "tokenisfullstring";
    let expected = (Token::Ident(String::from("tokenisfullstring")), "");
    assert_eq!(expected, read_ident(test_string));
}

#[test]
fn test_read_int_small() {
    let test_string = "1234 test";
    let expected = (Token::Int(1234), " test");
    assert_eq!(expected, read_int(test_string));
}

#[test]
fn test_read_int_one_char() {
    let test_string = "1 2 3";
    let expected = (Token::Int(1), " 2 3");
    assert_eq!(expected, read_int(test_string));
}

#[test]
fn test_read_int_full_string() {
    let test_string = "1234";
    let expected = (Token::Int(1234), "");
    assert_eq!(expected, read_int(test_string));
}
