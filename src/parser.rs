use crate::lexer::Token;

// Statements
pub enum Stmnt {
    LET(String, Exp),       // let <id> = <exp>
    RETURN(Exp),            // return <exp>
    EXPRESSION(Exp),        // <exp>
    BLOCK(Vec<Box<Stmnt>>), // { <stmnt>; ... }
}

// Expressions
pub enum Exp {
    IDENT(String),                         // <id>
    INT(usize),                            // <int>
    BOOL(bool),                            // true | false
    PREFIXOP(UnaryOp, Box<Exp>),           // [!-] <exp>
    INFIXOP(Box<Exp>, BinaryOp, Box<Exp>), // <exp> <op> <exp>
    IF(Box<Exp>, Box<Stmnt>, Box<Stmnt>),  // if <exp> <stmnt> (else <stmnt>)?
    FN(Vec<Box<Exp>>, Vec<Box<Stmnt>>),    // fn (<exp>, ...) { <stmnt>; ... }
    CALL(Box<Exp>, Vec<Box<Exp>>),         // <exp> (<exp>, ...)
}

// Unary operators
pub enum UnaryOp {
    BANG,  // !
    MINUS, // -
}

// Binary operators
pub enum BinaryOp {
    EQ,       // ==
    NOTEQ,    // !=
    LT,       // <
    GT,       // >
    PLUS,     // +
    MINUS,    // -
    SLASH,    // /
    ASTERISK, // *
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmnt> {
    unimplemented!()
}

fn parse_stmnt(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_stmnt_let(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_stmnt_return(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_stmnt_expression(tokens: &Vec<Token>) -> (Stmnt, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_ident(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_int(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_bool(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_prefix_op(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}

fn parse_exp_infix_op(tokens: &Vec<Token>, prec: u8) -> (Exp, &Vec<Token>) {
    unimplemented!()
}
