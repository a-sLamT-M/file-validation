use crate::command_handler::clis::cli::{FileValidationCli, FileValidationCliCommands};
use crate::command_handler::file_validation_actions::FileValidationActions;
use crate::file::file_hash::FileHash;
use clap::Error;
use clap::Parser;

pub(crate) fn matcher() -> Result<String, Error> {
    let parsed = FileValidationCli::parse();
    let mut actions: FileValidationActions = FileValidationActions::new();
    let mut result = Vec::new();
    let mut is_import: bool = false;
    match &parsed.opt {
        FileValidationCliCommands::Import => {
            result = actions.import(&parsed.path)?;
            is_import = true;
        }
        FileValidationCliCommands::Export => {
            actions.export(&parsed.path)?;
        }
    };
    if result.len() > 0 {
        let missing_msg = gen_missing_msg(result);
        return Ok(missing_msg);
    }
    if is_import {
        return Ok("\nAll files verified.".to_string());
    }
    return Ok("\nDone".to_string());
}

pub fn gen_missing_msg(v: Vec<&FileHash>) -> String {
    let mut msg_builder = String::from(format!("\n{} file(s) failed validation. \n", v.len()));
    for x in v {
        msg_builder.push_str(&format!("at {}\n", x.get_rel_path()));
    }
    msg_builder
}
