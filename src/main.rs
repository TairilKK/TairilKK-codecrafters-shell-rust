#[allow(unused_imports)]
use std::io::{self, Write};

use std::collections::HashMap;

fn main() {
    let commands = command_registration();

    loop {
        let (command, args): (String, String) = read();
        eval(&commands, command, args);
    }
}

trait Command {
    fn execute(&self, args: String) -> Result<(), ()>;
}

struct Type {
    commands: Vec<String>,
}
impl Command for Type {
    fn execute(&self, args: String) -> Result<(), ()> {
        match self.commands.contains(&args) {
            true => println!("{} is a shell builtin", args),
            false => println!("{}: not found", args),
        }

        Ok(())
    }
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

fn command_registration() -> HashMap<String, Box<dyn Command>> {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

    commands.insert("exit".into(), Box::new(Exit));
    commands.insert("echo".into(), Box::new(Echo));

    let mut names: Vec<String> = commands.keys().cloned().collect();
    names.push("type".into());

    commands.insert("type".into(), Box::new(Type { commands: names }));

    commands
}

fn eval(commands: &HashMap<String, Box<dyn Command>>, command: String, args: String) {
    if let Some(cmd) = commands.get(&command) {
        let _ = cmd.execute(args);
    } else {
        println!("{}: command not found", command);
    }
}
