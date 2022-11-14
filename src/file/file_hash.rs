use std::{fs, io};
use sha2::{Digest, Sha512};
use std::path::Path;

pub struct FileHash {
    hash: String,
    path: String,
    rel_path: String,
}

impl FileHash {
    pub fn new(filepath: &str, folder_path: &str) -> std::io::Result<FileHash> {
        let mut file = fs::File::open(filepath)?;
        let mut hasher = Sha512::new();
        let _ = io::copy(&mut file, &mut hasher)?;
        let hash_result = hasher.finalize();
        let hash_str = format!("{:X}", hash_result);

        let result = FileHash {
            hash: hash_str,
            path: String::from(filepath),
            rel_path: Self::gen_rel_path(filepath, folder_path),
        };
        Ok(result)
    }

    pub fn load(path: &str, hash: &str, folder_path: &str) -> FileHash {
        FileHash {
            hash: String::from(hash),
            path: String::from(path),
            rel_path: Self::gen_rel_path(path, folder_path),
        }
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_rel_path(&self) -> &str {
        &self.rel_path
    }

    fn gen_rel_path(filepath: &str, folder_path: &str) -> String {
        let rel_path = Path::new(filepath)
            .strip_prefix(folder_path)
            .unwrap_or_else(|_| Path::new(filepath))
            .to_str()
            .unwrap();
        String::from(rel_path)
    }
}

impl PartialEq<Self> for FileHash {
    fn eq(&self, other: &Self) -> bool {
        &self.hash == &other.hash && &self.rel_path == &other.rel_path
    }
}

impl Eq for FileHash {}
