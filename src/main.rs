use std::io;
use std::io::Write;
use std::process;

mod calc;

use calc::lexer::Lexer;
use calc::token::Token;

fn main() {
    lexer_main();
}

fn get_expression() -> String {
    let mut input = String::new();

    print!("> ");
    match io::stdout().flush() {
        Ok(x) => x,
        Err(e) => {
            println!("Unable to flush stdout: {}", e);
            process::exit(1);
        }
    };

    match io::stdin().read_line(&mut input) {
        Ok(x) => x,
        Err(e) => {
            println!("Unable to read from stdin: {}", e);
            process::exit(1);
        }
    };

    String::from(input.trim_end_matches("\n"))
}

#[allow(dead_code)]
fn lexer_main() {
    loop {
        let input = get_expression();
        if input == String::from("exit") {
            break;
        }

        let mut lexer = Lexer::new(input.as_str());
        loop {
            let result = lexer.next_token();
            match result {
                Ok(token) => {
                    println!("{:?}", token);
                    match token {
                        Token::EOF(_) => break,
                        _ => {}
                    };
                }
                Err(error) => {
                    eprintln!("Error: {:?}", error);
                    break;
                }
            };
        }
    }
}
