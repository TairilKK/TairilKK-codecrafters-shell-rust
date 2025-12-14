use std::env;

use super::traits::Command;

pub struct Cd;

impl Command for Cd {
    fn execute(&self, args: String) -> Result<(), ()> {
        if let Err(_) = env::set_current_dir(&args) {
            println!("cd: {}: No such file or directory", args);
        }
        Ok(())
    }
}
