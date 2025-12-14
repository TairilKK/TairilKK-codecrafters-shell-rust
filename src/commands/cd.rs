use std::env;

use super::traits::Command;

pub struct Cd;

impl Command for Cd {
    fn execute(&self, args: String) -> Result<(), ()> {
        if args.contains('~') {
            if let Ok(v) = env::var("HOME") {
                let args = args.replace("~", v.as_str());
                if let Err(_) = env::set_current_dir(&args) {
                    println!("cd: {}: No such file or directory", args);
                }
            } else {
                println!("cd: {}: No such file or directory", args);
            }
        } else {
            if let Err(_) = env::set_current_dir(&args) {
                println!("cd: {}: No such file or directory", args);
            }
        }

        Ok(())
    }
}
