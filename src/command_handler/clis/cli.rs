use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum FileValidationCliCommands {
    Export,
    Import,
}

#[derive(Parser)]
#[command(name = "File Validation")]
#[command(author = "Omega")]
#[command(version = "1.0")]
#[command(about = "Verify file integrity", long_about = None)]
pub struct FileValidationCli {
    #[command(subcommand)]
    pub opt: FileValidationCliCommands,

    // Change the working directory. Default is the current directory of shell.
    #[arg(short, long)]
    pub path: Option<String>,
}
