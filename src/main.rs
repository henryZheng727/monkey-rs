mod lexer;
mod parser;

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

        // parse input

        // print results
        println!("{:?}", tokens);
    }
}
