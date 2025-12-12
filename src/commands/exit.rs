use super::traits::Command;

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _: String) -> Result<(), ()> {
        std::process::exit(0);
    }
}
