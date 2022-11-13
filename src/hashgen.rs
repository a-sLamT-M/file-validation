use std::io::{Error, ErrorKind};
use std::path::{Path};
use walkdir::WalkDir;
use crate::file::file_hash::FileHash;

pub struct HashGen {
    file_hash_vec: Vec<FileHash>,
    folder_path: String
}

impl HashGen {
    pub fn new() -> HashGen {
        HashGen {
            file_hash_vec: Vec::new(),
            folder_path: String::from(std::env::current_dir().unwrap().into_os_string().into_string().unwrap())
        }
    }

    pub fn set_path(&mut self, path: &str) -> Result<(), Error> {
        if Path::new(path).is_dir() {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        self.folder_path = String::from(path);
        Ok(())
    }

    pub fn get_hash_vec(&self) -> &Vec<FileHash> {
        &self.file_hash_vec
    }

    pub fn gen_file_hash_map(&mut self) -> Result<(), Error> {
        let p = Path::new(&self.folder_path);
        if p.is_dir() {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        for file in WalkDir::new(p).into_iter() {
            match file {
                Ok(x) => {
                    self.file_hash_vec.push(FileHash::new(x.path().to_str().unwrap())
                        .expect("File path cannot convert to string."));
                }
                Err(_e) => { continue; }
            }
        }
        Ok(())
    }
}

