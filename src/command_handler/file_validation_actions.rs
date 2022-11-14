use crate::file::file_hash::FileHash;
use crate::file::hash_map_file::HashMapFile;

pub struct FileValidationActions {
    hash_gen: HashMapFile,
    other_hash_gen: HashMapFile,
}

impl FileValidationActions {
    pub fn new() -> FileValidationActions {
        FileValidationActions {
            hash_gen: HashMapFile::new(),
            other_hash_gen: HashMapFile::new(),
        }
    }

    pub fn export(&mut self, folder_path: &Option<String>) -> std::io::Result<String> {
        self.init_hash_gen(&folder_path)?;
        self.hash_gen.save_to_file()?;
        Ok(String::from(format!("File saved.")))
    }

    pub fn import(&mut self, folder_path: &Option<String>) -> std::io::Result<Vec<&FileHash>> {
        self.init_hash_gen(folder_path)?;
        self.other_hash_gen.load()?;
        match self.hash_gen.compare(&self.other_hash_gen) {
            None => Ok(Vec::new()),
            Some(x) => Ok(x),
        }
    }

    fn init_hash_gen(&mut self, folder_path: &Option<String>) -> std::io::Result<()> {
        if let Some(x) = folder_path {
            self.hash_gen.set_folder_path(x.as_str())
        };
        self.hash_gen.gen_file_hash_map()
    }
}
