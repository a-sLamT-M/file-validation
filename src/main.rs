use crate::command_handler::matcher::file_validation_match;

mod file;
mod command_handler;
mod hashgen;
mod const_def;

fn main() {
    match file_validation_match::matcher() {
        Ok(x) => {
            println!("{}", x);
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    }
}
