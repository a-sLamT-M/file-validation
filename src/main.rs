use crate::command_handler::matcher::file_validation_match;

mod command_handler;
mod const_def;
mod file;

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
