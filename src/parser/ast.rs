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
