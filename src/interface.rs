use crate::config::global::*;
use crate::commands::*;

use phoenix_shell::*;

use std::f64::consts::E;
use std::io::{stdout, stdin, Write};
use std::process::Command;

pub struct Interface;

impl Interface{
    pub fn init(){
        let history_file = history::check_history_file();

        if history_file == true {
            history::read_history_file();
        }
        else{
            history::create_history_file();
        }
        Self::wait_for_command();
    }

    pub fn display(){
        print!("User: C:/Windows/User>");
        stdout().flush().unwrap();
    }
    pub fn wait_for_command(){
        loop{
            let mut input = String::new();

            Self::display();

            stdin().read_line(&mut input).expect("Failed to read command");

            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => cd::move_to_dir(),
                "exit" => return,
                command => {
                    let child = Command::new(command)
                    .args(args)
                    .spawn();

                    match child {
                        Ok(mut child) => {child.wait().unwrap();}
                        Err(e) => {
                            eprintln!("{} {}", RED ,e)
                        }

                        
                    }

                }
            }

            
        }
        

    }


}
