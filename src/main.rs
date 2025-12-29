mod lexer;
mod parser;

use crate::lexer::token::Token;
use std::io::Write;

fn main() {
    loop {
        // display the prompt
        let _ = print!(">> ");
        let _ = std::io::stdout().flush();

        // read input
        let mut input = String::new();
        let read_bytes = std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot open interactive terminal.");

        // close REPL on EOF
        if read_bytes <= 0 {
            break;
        }

        // lex input and validate tokens
        let tokens = lexer::lex(&input);
        let mut lex_error = false;

        for token in tokens.iter() {
            match token {
                Token::ILLEGAL(char) => {
                    lex_error = true;
                    println!("Illegal character: {char}");
                }
                _ => continue,
            }
        }

        if lex_error {
            continue;
        }

        // print results
        println!("{:?}", tokens);
    }
}
