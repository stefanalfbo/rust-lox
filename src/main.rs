use crate::scanner::Scanner;

mod scanner;
mod token;
mod token_type;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        println!("Usage: rust-lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    use std::fs;
    let source = fs::read_to_string(path).expect("Could not read file");
    run(&source);

    // if had_error {
    //     std::process::exit(65);
    // }
}

fn run_prompt() {
    use std::io::{self, Write};
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                run(&line);
                // had_error = false;
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    // had_error = true;
}
