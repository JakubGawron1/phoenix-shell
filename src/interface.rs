use std::env;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Interface {
    pub(crate) current_dir: PathBuf,
    pub(crate) is_admin: bool,

}

impl Interface {
    // First public functions

    // Init shell
    pub fn init(&mut self) {
        println!("Hello in Phoenix Shell v{}", env!("CARGO_PKG_VERSION"));
        Self::display(self);
        self.is_admin = crate::config::admin::check_admin();
        Self::wait_for_command(self);
    }
    // Display current location
    pub fn display(&mut self) {
        let binding = env::current_dir().unwrap();
        let working_dir = binding.display();

        if self.is_admin == true {
            print!("Root {}>", working_dir)
        }
        else{
            print!("User {}>", working_dir)
        }
        stdout().flush().unwrap();
        return;
    }
    // Change current dir
    fn current_dir(&self) {}
    // Wait for commands
    fn wait_for_command(&mut self) {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let command = input.trim();

            let mut child = Command::new(command)
            .spawn()
            .unwrap();

            child.wait().unwrap();
            Self::display(self);

        }
    }
}
