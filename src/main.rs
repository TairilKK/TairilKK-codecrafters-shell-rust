use codecrafters_shell::*;

fn main() {
    let commands = command_registration();

    loop {
        let (command, args): (String, String) = read();
        eval(&commands, command, args);
    }
}
