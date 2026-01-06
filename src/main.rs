use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    process,
};

mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

pub struct Jlox {
    had_error: bool,
}

impl Jlox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        self.run(&contents);

        if self.had_error {
            process::exit(65);
        }
        Ok(())
    }

    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("> ");
            stdout.flush().unwrap();

            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    self.run(&line);
                    self.had_error = false;
                }
                Err(e) => {
                    eprintln!("입력 오류: {}", e);
                    break;
                }
            }
        }
    }

    fn run(&mut self, source: &str) {
        // 스캐닝, 파싱, 인터프리팅...
        println!("실행: {}", source);
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: i32, pos: &str, message: &str) {
        eprintln!("[line {}] Error {} : {}", line, pos, message);
        self.had_error = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut jlox = Jlox::new();

    match args.len() {
        1 => jlox.run_prompt(),
        2 => {
            if let Err(e) = jlox.run_file(&args[1]) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("사용법: jaylox [scripts]");
            process::exit(64);
        }
    }
}
