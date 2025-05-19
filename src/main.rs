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
    const EXIT_MSG: &str = ".exit";
    println!("Use '{}' to exit!", EXIT_MSG);

    loop {
        let input = get_expression();
        if input == EXIT_MSG {
            break;
        }

        let mut lexer = Lexer::new(input);
        loop {
            let result = lexer.next_token();
            match result {
                Ok(token) => {
                    println!("{:?}", token);
                    if let Token::EOF = token {
                        break;
                    }
                }
                Err(error) => {
                    eprintln!("Error: {:?}", error);
                    break;
                }
            };
        }
    }
}

// #[allow(dead_code)]
// fn parser_main() {
//     loop {
//         let mut input = get_expression();
//         if input == String::from("exit\n") {
//             break;
//         }

//         let lexer = Lexer::new(input.as_str());
//         let tokens = lexer.lex();

//         let mut parser = Parser::new(tokens);
//         let tree = parser.parse();

//         match tree {
//             Ok(t) => {
//                 println!("Tree:");
//                 println!("{:?}", t);
//             },
//             Err(e) => {
//                 println!("Parsing failed: {:?}", e);
//                 break;
//             }
//         };
//     }
// }

// #[allow(dead_code)]
// fn eval_main() {
//     loop {
//         let mut input = get_expression();
//         if input == String::from("exit\n") {
//             break;
//         }

//         let mut lexer = Lexer::new(expr.as_str());
//         let tokens = lexer.lex();

//         let mut parser = Parser::new(tokens);
//         let tree = parser.parse();

//         match tree {
//             Err(e) => {
//                 println!("Parsing failed: {:?}", e);
//                 break;
//             },
//             Ok(t) => {
//                 let eval = Eval::new(&tree);
//                 let value = eval.evaluate();

//                 match value {
//                     Value::INT(val) => println!("{val}"),
//                 }
//             }
//         };
//     }
// }
