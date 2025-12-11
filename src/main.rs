#[allow(unused_imports)]
use std::io::{self, Write};

use std::collections::HashMap;

fn main() {
    loop {
        let (command, args): (String, String) = read();
        eval(command, args);
    }
}

trait Command {
    fn execute(&self, args: String) -> Result<(), ()>;
}
struct Echo;
impl Command for Echo {
    fn execute(&self, args: String) -> Result<(), ()> {
        println!("{}", args);
        Ok(())
    }
}

struct Exit;
impl Command for Exit {
    fn execute(&self, _: String) -> Result<(), ()> {
        std::process::exit(0);
    }
}

fn read() -> (String, String) {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input_buffer = String::new();
    let _ = io::stdin()
        .read_line(&mut input_buffer)
        .expect("Cannot read input.");

    // Removes '\n'
    input_buffer.pop();

    input_buffer
        .split_once(' ')
        .map(|(l, r)| (l.to_string(), r.to_string()))
        .unwrap_or((input_buffer, "".to_string()))
}

fn eval(command: String, args: String) {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
    commands.insert("exit".to_string(), Box::new(Exit));
    commands.insert("echo".to_string(), Box::new(Echo));

    if let Some(cmd) = commands.get(&command) {
        let _ = cmd.execute(args);
    } else {
        println!("{}: command not found", command);
    }
}
