use std::io::{self, Write};
use crate::lexer::Lexer;

pub fn repl() {
    loop {
        print!("> ");
        // Flush stdout to ensure the prompt is displayed
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim the input to remove any trailing newline characters
        let input = input.trim();

        // Check for exit command
        if input == "exit" || input == "quit" {
            break;
        }

        // Evaluate the input
        let result = evaluate(input);

        // Print the result
        println!("{}", result);
    }
}

fn evaluate(input: &str) -> String {
    let mut lexer = Lexer::new(input.to_string());
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());

        if token.token_type == crate::token::TokenType::EOF {
            break;
        }
    }
    tokens.iter().map(|t| format!("{:?}", t)).collect::<Vec<String>>().join("\n")
}
