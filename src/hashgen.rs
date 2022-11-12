use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::cli::Cli;
use crate::command_handler::handler::CmdHandler;
use crate::file::filehash::FileHash;

pub struct HashGen {
    file_hash_vec: Vec<FileHash>,
    folder_path: PathBuf
}

impl HashGen {
    pub fn new() -> HashGen {
        HashGen {
            file_hash_vec: Vec::new(),
            folder_path: std::env::current_dir().unwrap()
        }
    }

    pub fn set_path(&mut self, path: &PathBuf) -> Result<_, Error> {
        if path.is_dir() {
            Err(Error::from(ErrorKind::InvalidInput))
        }
        self.folder_path = path.clone();
        Ok(())
    }

    pub fn get_hash_vec(&self) -> &Vec<FileHash> {
        &self.file_hash_vec
    }

    fn gen_file_hash_map(&mut self) -> Result<_, Error> {
        if self.folder_path.is_dir() {
            Err(Error::from(ErrorKind::InvalidInput))
        }
        for file in WalkDir::new(&self.folder_path).into_iter() {
            match file{
                Ok(x) => {
                    file_hash_vec.push(FileHash::new(x.path().to_str()?)
                        .expect("File path cannot convert to string."));
                }
                Err(_) => {}
            }
        }
        Ok(())
    }
}

