// Statements
#[derive(Debug, PartialEq, Clone)]
pub enum Stmnt {
    Let(String, Exp),  // let <id> = <exp>
    Return(Exp),       // return <exp>
    Expression(Exp),   // <exp>
    Block(Vec<Stmnt>), // { <stmnt>; ... }
    Illegal,           // failed to parse
}

// Expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Exp {
    Ident(String),                         // <id>
    Int(usize),                            // <int>
    Bool(bool),                            // true | false
    PrefixOp(UnaryOp, Box<Exp>),           // [!-] <exp>
    InfixOp(Box<Exp>, BinaryOp, Box<Exp>), // <exp> <op> <exp>
    If(Box<Exp>, Box<Stmnt>, Box<Stmnt>),  // if <exp> <stmnt> (else <stmnt>)?
    Fn(Vec<Exp>, Vec<Stmnt>),              // fn (<exp>, ...) { <stmnt>; ... }
    Call(Box<Exp>, Vec<Exp>),              // <exp> (<exp>, ...)
    Illegal,                               // failed to parse
}

#[derive(Debug, PartialEq, Clone)]
// Unary operators
pub enum UnaryOp {
    Bang,  // !
    Minus, // -
}

#[derive(Debug, PartialEq, Clone)]
// Binary operators
pub enum BinaryOp {
    Eq,       // ==
    NotEq,    // !=
    Lt,       // <
    Gt,       // >
    Plus,     // +
    Minus,    // -
    Slash,    // /
    Asterisk, // *
}
