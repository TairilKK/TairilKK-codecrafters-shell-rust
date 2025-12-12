use std::collections::HashMap;
use std::io::{self, Write};
use std::process;

pub mod commands;
use crate::commands::{echo::Echo, exit::Exit, pwd::Pwd, traits::Command, type_cmd::Type};

pub fn eval(commands: &HashMap<String, Box<dyn Command>>, command: String, args: String) {
    if let Some(cmd) = commands.get(&command) {
        let _ = cmd.execute(args);
    } else {
        if let Ok(output) = process::Command::new(&command)
            .args(args.split(' '))
            .output()
        {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("{}: command not found", command);
        }
    }
}

pub fn read() -> (String, String) {
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

pub fn command_registration() -> HashMap<String, Box<dyn Command>> {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

    commands.insert("exit".into(), Box::new(Exit));
    commands.insert("echo".into(), Box::new(Echo));
    commands.insert("pwd".into(), Box::new(Pwd));

    let mut names: Vec<String> = commands.keys().cloned().collect();
    names.push("type".into());

    commands.insert("type".into(), Box::new(Type::new(names)));

    commands
}
