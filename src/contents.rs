use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::errors::NotAvailableError;

enum FileState {
    None,
    CodeBlock,
    Sentence,
    Meta,
}

pub struct File {
    pub path: String,
    pub contents: String,
    pub sentences: Vec<String>,
}

impl File {
    pub fn new(path: String, contents: String) -> Self {
        Self {
            path,
            contents,
            sentences: Vec::new(),
        }
    }
}

fn has_file_extension(path: &PathBuf, ending: &str) -> bool {
    if let Some(path) = path.to_str() {
        return path.ends_with(ending);
    }

    false
}

pub fn load_files_from_dir(dir: PathBuf, prefix: &PathBuf, ending: &str) -> Result<Vec<File>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let mut sub_files = load_files_from_dir(path, &prefix, ending)?;
            files.append(&mut sub_files);
        } else if path.is_file() && has_file_extension(&path, ending) {
            let path = Path::new(&path).strip_prefix(prefix)?.to_owned();
            println!("Path: {:?}", path);
            let contents = fs::read_to_string(&path)?;
            let key = path.to_str().ok_or(NotAvailableError {})?;
            let file = File::new(key.to_string(), contents);
            files.push(file);
        }
    }
    Ok(files)
}
