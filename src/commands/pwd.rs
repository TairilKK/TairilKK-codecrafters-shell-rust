use super::traits::Command;
use std::env;

pub struct Pwd;

impl Command for Pwd {
    fn execute(&self, _: String) -> Result<(), ()> {
        if let Ok(path) = env::current_dir() {
            println!("{}", path.display());
        }

        Ok(())
    }
}
