use clap::Error;
use crate::command_handler::file_validation_actions::FileValidationActions;
use crate::command_handler::clis::cli::{FileValidationCli, FileValidationCliCommands};
use crate::file::file_hash::FileHash;
use clap::Parser;

pub(crate) fn matcher() -> Result<String, Error> {
    let parsed = FileValidationCli::parse();
    let mut actions: FileValidationActions = FileValidationActions::new();
    let mut result = Vec::new();
    match &parsed.opt {
        FileValidationCliCommands::Import{path} => {
            result = actions.import(&parsed.path, path.as_str())?;
        },
        FileValidationCliCommands::Export => {
            actions.export(&parsed.path)?;
        }
    };
    if result.len() > 0 {
        let missing_msg = gen_missing_msg(result);
        return Ok(missing_msg);
    }
    Ok("All files verified.".to_string())
}

pub fn gen_missing_msg(v: Vec<&FileHash>) -> String {
    let mut msg_builder = String::from(format!("{} files failed validation.", v.len()));
    for x in v {
        msg_builder.push_str(&format!("at {}\n", x.get_path()));
    }
    msg_builder
}