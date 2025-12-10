#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let input_buffer: String = read();
        println!("{}: command not found", input_buffer);
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
