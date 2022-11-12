use std::collections::HashMap;
use std::f32::consts::E;
use std::fs;
use std::path::{Path, PathBuf};
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

pub struct FileHash {
    hash: Vec<u8>,
    file: Vec<u8>,
    path: String
}

impl FileHash {
    pub fn new(filepath: &str) -> std::io::Result<FileHash> {
        let bytes: Vec<u8> = if let Ok(t) = fs::read(&filepath) {
            t
        } else {
            Err(E)
        };
        let mut hasher = Sha512::new();
        hasher.update(&bytes);
        let hashResult = hasher.finalize();
        let result = FileHash {
            hash: hashResult.to_vec(),
            file: bytes,
            path: String::from(filepath)
        };
        Ok(result)
    }

    pub fn get_hash_str(&self) -> std::io::Result<String> {
        str::from_utf8(&self.hash)
    }
}