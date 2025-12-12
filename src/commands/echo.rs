use super::traits::Command;

pub struct Echo;

impl Command for Echo {
    fn execute(&self, args: String) -> Result<(), ()> {
        println!("{}", args);
        Ok(())
    }
}
