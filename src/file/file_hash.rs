use std::fs;
use sha2::{ Sha512, Digest};

pub struct FileHash {
    hash: String,
    path: String
}

impl FileHash {
    pub fn new(filepath: &str) -> std::io::Result<FileHash> {
        let bytes: Vec<u8> = fs::read(&filepath)?;
        let mut hasher = Sha512::new();
        hasher.update(&bytes);
        let hashResult = hasher.finalize();
        let hash_str = format!("{:X}", hashResult);

        let result = FileHash {
            hash: hash_str,
            path: String::from(filepath)
        };
        Ok(result)
    }

    pub fn load(hash: &str, path: &str) -> FileHash {
        FileHash {
            hash: String::from(hash),
            path: String::from(path)
        }
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl PartialEq<Self> for FileHash {
    fn eq(&self, other: &Self) -> bool {
        &self.hash == &other.hash
    }
}

impl Eq for FileHash {}