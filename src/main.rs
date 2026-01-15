mod lexer;
mod parser;

use crate::lexer::token::Token;
use std::io::Write;

fn main() {
    loop {
        // display the prompt
        print!(">> ");
        std::io::stdout()
            .flush()
            .expect("Cannot open interactive terminal.");

        // read input
        let mut input = String::new();
        let read_bytes = std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot open interactive terminal.");

        // close REPL on EOF
        if read_bytes == 0 {
            break;
        }

        // lex input and validate tokens
        let tokens = lexer::lex(&input);
        let mut lex_error = false;

        for token in tokens.iter() {
            match token {
                Token::Illegal(char) => {
                    lex_error = true;
                    println!("Illegal character: {char}");
                }
                _ => continue,
            }
        }

        if lex_error {
            continue;
        }

        // parse tokens
        let program = parser::parse(tokens);

        // print results
        println!("{:?}", program);
    }
}
