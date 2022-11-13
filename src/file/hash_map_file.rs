use std::env::current_dir;
use std::fs::File;
use std::io;
use std::io::{BufRead, Error, ErrorKind, LineWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::file::file_hash::FileHash;
use crate::const_def::const_str;
use crate::const_def::const_str::HASHMAP_SPLIT;

pub struct HashMapFile {
    ext: String,
    filename: String,
    folder_path: String,
    hashmap: Vec<FileHash>
}

impl HashMapFile {
    pub fn new() -> HashMapFile {
        HashMapFile {
            ext: const_str::HASHMAP_FILE_EXT.to_string(),
            filename: String::from(current_dir().unwrap().file_name().unwrap().to_str().unwrap()),
            folder_path: String::new(),
            hashmap: vec![]
        }
    }

    pub fn load(&mut self, path: &str) -> Result<(), Error> {
        self.hashmap.clear();
        self.filename = String::from(Path::new(path).file_name().expect("Can not read file/dir name.").to_str().unwrap());
        let p = Path::new(path);
        if p.is_dir() || !p.exists() || !(p.extension().unwrap().to_str().unwrap().to_string() == const_str::HASHMAP_FILE_EXT) {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        let file = File::open(p);
        let mut hash_map: Vec<FileHash> = Vec::new();
        for l in io::BufReader::new(file?).lines() {
            let mut str_result: String = String::new();
            let mut split: Vec<&str> = Vec::new();
            match l {
                Ok(x) => {
                    str_result = x;
                }
                Err(_) => {}
            }
            if str_result.is_empty() {
                continue;
            }
            for str in str_result.split(const_str::HASHMAP_SPLIT) {
                split.push(str)
            }
            if split.len() != 2 {
                continue;
            }
            let new_hash = FileHash::load(split[0], split[1]);
            hash_map.push(new_hash);
        }
        self.hashmap = hash_map;
        Ok(())
    }

    pub fn set_folder_path(&mut self, path: &str) {
        self.folder_path = path.to_string()
    }

    pub fn get_hash_vec(&self) -> &Vec<FileHash> {
        &self.hashmap
    }

    pub fn gen_file_hash_map(&mut self) -> Result<(), Error> {
        self.hashmap.clear();
        let p = Path::new(&self.folder_path);
        if p.is_dir() {
            Err(Error::from(ErrorKind::InvalidInput)).unwrap()
        }
        for file in WalkDir::new(p).into_iter() {
            match file {
                Ok(x) => {
                    self.hashmap.push(FileHash::new(x.path().to_str().unwrap())
                        .expect("File path cannot convert to string."));
                }
                Err(_) => {}
            }
        }
        Ok(())
    }

    pub fn save_to_file(&self, save_path: Option<&str>) -> std::io::Result<&Self> {
        let sp = if let Some(x) = save_path {
            String::from(x)
        } else {
            self.folder_path.to_string().clone()
        };
        let mut sp = PathBuf::from(sp);
        sp.push(&self.filename);
        sp.push(&self.ext);
        let file = File::create(sp).expect("Unable to create files.");
        let mut file = LineWriter::new(file);
        for h in &self.hashmap {
            let line = String::from(format!("{}{}{}",&h.get_path(), HASHMAP_SPLIT, &h.get_hash()));
            file.write( line.as_bytes())?;
        }
        Ok(self)
    }

    pub fn compare<'a>(&'a self, input: &'a Self) -> Option<Vec<&FileHash>> {
        let result: Vec<&FileHash> = input.hashmap.iter()
            .filter(|x| !self.hashmap.iter().any(|i| &i == x))
            .collect();
        if result.len() <= 0 {
            None
        } else {
            Some(result)
        }
    }
}