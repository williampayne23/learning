use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn repl(eval: fn(&str) -> String) {
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
        let result = eval(input);

        // Print the result
        println!("{}", result);
    }
}

fn lex(input: &str) -> String {

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

fn parse(input: &str) -> String {
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if parser.has_errors() {
        let mut s = format!("Parser errors:\n");
        for e in parser.errors().iter() {
            s.push_str(&format!("\t{}\n", e));
        }
        return s;
    }


    format!("{:?}", program)
}

pub fn repl_command() -> seahorse::Command {
    seahorse::Command::new("repl")
        .description("Starts the REPL")
         .flag(
            seahorse::Flag::new("step", seahorse::FlagType::String)
                .description("Which step to run")
                .alias("s"),
        )
        .action(|c| {
            if let Ok(step) = c.string_flag("step") {
                match step.as_str() {
                    "lexer" => repl(lex),
                    "lex" => repl(lex),
                    "parser" => repl(parse),
                    "parse" => repl(parse),
                    _ => println!("Invalid step"),
                }
                return;
            }
            println!("You need to provide a step");
        })
}
