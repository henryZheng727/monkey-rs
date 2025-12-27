mod lexer;

use std::io::Write;

fn main() {
    loop {
        // display the prompt
        print!(">> ");
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

        // lex input
        let tokens = lexer::lex(&input);

        // print results
        println!("{:?}", tokens);
    }
}
