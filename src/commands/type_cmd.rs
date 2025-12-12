use std::{
    env,
    fs::{self, DirEntry},
    os::unix::fs::PermissionsExt,
};

use super::traits::Command;

const SEPARATOR: char = ':';

pub struct Type {
    commands: Vec<String>,
}

impl Type {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }

    fn is_command(&self, name: &String) -> bool {
        self.commands.contains(name)
    }

    pub fn is_executable(name: &String) -> Option<String> {
        let dirs = Self::get_path_dirs()?;

        for dir in dirs {
            if let Some(path) = Self::search_dir(&dir, name) {
                return Some(path);
            }
        }
        None
    }

    fn search_dir(dir: &String, name: &String) -> Option<String> {
        let paths = fs::read_dir(dir).ok()?;

        for path in paths {
            if let Ok(entry) = path {
                if Self::check_file_name(&entry, name) {
                    if let Some(path) = Self::check_permissions(&entry) {
                        return Some(path);
                    }
                }
            }
        }

        None
    }

    fn check_permissions(dir_entry: &DirEntry) -> Option<String> {
        if let Ok(metadata) = fs::metadata(dir_entry.path()) {
            let permissions = metadata.permissions();

            if permissions.mode() & 0o111 != 0 {
                if let Some(str) = dir_entry.path().to_str() {
                    return Some(str.to_string());
                }
            }
            return None;
        }
        None
    }

    fn check_file_name(dir_entry: &DirEntry, name: &String) -> bool {
        if let Ok(path) = dir_entry.file_name().into_string() {
            return path == *name;
        }
        false
    }

    fn get_path_dirs() -> Option<Vec<String>> {
        let dirs = match env::var("PATH") {
            Ok(v) => v,
            Err(_) => return None,
        };

        if dirs.is_empty() {
            None
        } else {
            Some(dirs.split(SEPARATOR).map(|v| v.to_string()).collect())
        }
    }
}

impl Command for Type {
    fn execute(&self, args: String) -> Result<(), ()> {
        match self.is_command(&args) {
            true => println!("{} is a shell builtin", args),
            false => match Type::is_executable(&args) {
                Some(path) => println!("{} is {}", args, path),
                None => println!("{}: not found", args),
            },
        }

        Ok(())
    }
}
