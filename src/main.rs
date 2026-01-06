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
    run(source);
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
            Ok(_) => run(line),
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn run(_source: String) {}
