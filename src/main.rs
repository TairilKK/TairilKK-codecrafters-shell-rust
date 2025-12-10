#[allow(unused_imports)]
use std::io::{self, Write};

use std::collections::HashMap;

fn main() {
    loop {
        let input_buffer: String = read();
        eval(input_buffer);
    }
}

trait Command {
    // fn name(&self) -> &'static str;
    fn execute(&self) -> Result<(), ()>;
}

struct Exit;

impl Command for Exit {
    /*
    fn name(&self) -> &'static str {
        "exit"
    }
    */
    fn execute(&self) -> Result<(), ()> {
        std::process::exit(0);
    }
}

fn read() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input_buffer = String::new();
    let _ = io::stdin()
        .read_line(&mut input_buffer)
        .expect("Cannot read input.");

    // Removes '\n'
    input_buffer.pop();
    input_buffer
}

fn eval(input: String) {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
    commands.insert("exit".to_string(), Box::new(Exit));

    if let Some(cmd) = commands.get(&input) {
        let _ = cmd.execute();
    } else {
        println!("{}: command not found", input);
    }
}
