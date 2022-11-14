use crate::const_def::const_str;
use crate::const_def::const_str::{HASHMAP_FILE_EXT, HASHMAP_SPLIT};
use crate::file::file_hash::FileHash;
use std::env::current_dir;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, LineWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct HashMapFile {
    ext: String,
    filename: String,
    folder_path: String,
    hashmap: Vec<FileHash>,
}

impl HashMapFile {
    pub fn new() -> HashMapFile {
        HashMapFile {
            ext: const_str::HASHMAP_FILE_EXT.to_string(),
            filename: String::from(
                current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ),
            folder_path: String::from(current_dir().unwrap().to_str().unwrap()),
            hashmap: vec![],
        }
    }

    pub fn load(&mut self) -> Result<(), Error> {
        let mut p = PathBuf::new();
        p.push(self.folder_path.as_str());
        p.push(
            Path::new(self.folder_path.as_str())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        p.set_extension(HASHMAP_FILE_EXT);
        self.hashmap.clear();
        self.filename = p.file_name().unwrap().to_str().unwrap().to_string();
        if !p.exists() {
            return Err(Error::new(ErrorKind::NotFound, "hmfv does not exist."));
        }
        let file = File::open(p);
        let mut hash_map: Vec<FileHash> = Vec::new();
        let mut lines = io::BufReader::new(file?).lines();
        let loaded_folder_path = lines.next().unwrap().unwrap();
        for l in lines {
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
            let new_hash = FileHash::load(split[0], split[1], loaded_folder_path.as_str());
            hash_map.push(new_hash);
        }
        self.hashmap = hash_map;
        Ok(())
    }

    pub fn set_folder_path(&mut self, path: &str) {
        self.folder_path = path.to_string();
        self.filename = Path::new(self.folder_path.as_str())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
    }

    pub fn gen_file_hash_map(&mut self) -> Result<(), Error> {
        self.hashmap.clear();
        let p = Path::new(&self.folder_path);
        if !p.is_dir() {
            Err(Error::from(ErrorKind::InvalidInput)).unwrap()
        }
        for file in WalkDir::new(p).into_iter() {
            match file {
                Ok(x) => {
                    println!("{}", x.path().to_str().unwrap());
                    if x.path().is_dir() || x.path().extension().unwrap() == HASHMAP_FILE_EXT {
                        continue;
                    }
                    self.hashmap.push(
                        FileHash::new(x.path().to_str().unwrap(), self.folder_path.as_str())
                            .expect("File path cannot convert to string."),
                    );
                }
                Err(_) => {}
            }
        }
        Ok(())
    }

    pub fn save_to_file(&self) -> std::io::Result<&Self> {
        let mut sp = PathBuf::from(&self.folder_path);
        sp.push(&self.filename);
        sp.set_extension(&self.ext);
        println!("{}", sp.to_str().unwrap());
        let file = File::create(sp).expect("Unable to create files.");
        let mut file = LineWriter::new(file);
        file.write(format!("{}\n", self.folder_path).as_bytes())?;
        for h in &self.hashmap {
            let line = String::from(format!(
                "{}{}{}\n",
                &h.get_path(),
                HASHMAP_SPLIT,
                &h.get_hash()
            ));
            file.write(line.as_bytes())?;
        }
        Ok(self)
    }

    pub fn compare<'a>(&'a self, input: &'a Self) -> Option<Vec<&FileHash>> {
        let result: Vec<&FileHash> = input
            .hashmap
            .iter()
            .filter(|x| !self.hashmap.iter().any(|i| &i == x))
            .collect();
        if result.len() <= 0 {
            None
        } else {
            Some(result)
        }
    }
}
