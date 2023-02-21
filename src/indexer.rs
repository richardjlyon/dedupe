//! Functionality for indexing a folder of images.
//!
//!

use std::{fs::DirEntry, path::PathBuf};

use walkdir::WalkDir;

use crate::error::AppError;

pub struct Indexer {
    root: String,
    pub filepaths: Vec<PathBuf>,
}

impl Indexer {
    /// Construct a new indexer
    pub fn new(root: String) -> Result<Self, AppError> {
        if !std::path::Path::new(&root).exists() {
            return Err(AppError::NetworkError);
        }
        let filepaths: Vec<PathBuf> = Vec::new();
        Ok(Self { root, filepaths })
    }

    /// Walk the given root and populate filepaths
    pub fn walk(&mut self) {
        let walker = WalkDir::new(&self.root).into_iter();
        for entry in walker {
            // get image files, assumed to be a result with an extension
            let entry = entry.unwrap();
            if is_valid(&entry) {
                self.filepaths.push(entry.into_path())
            }
        }
    }

    /// Compute the number of file paths
    pub fn n_paths(&self) -> u64 {
        self.filepaths.len() as u64
    }
}

/// return true if the entry is a valid image
/// for now, this is if it has an extension
fn is_valid(entry: &walkdir::DirEntry) -> bool {
    entry.clone().into_path().extension().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let good_root = String::from("/Users/richardlyon/Dev/rust/dedupe/images");
        let indexer = Indexer::new(good_root);
        assert!(indexer.is_ok());

        let bad_root = String::from("/BadRoot");
        let indexer = Indexer::new(bad_root);
        assert!(indexer.is_err());
    }

    #[test]
    fn it_walks() {
        let root = String::from("/Users/richardlyon/Dev/rust/dedupe/images");
        let mut indexer = Indexer::new(root).unwrap();

        indexer.walk();

        assert_eq!(indexer.filepaths.len(), 131);
    }

    #[test]
    fn it_computes_number_of_paths() {
        let root = String::from("/Users/richardlyon/Dev/rust/dedupe/images");
        let mut indexer = Indexer::new(root).unwrap();

        indexer.walk();

        assert_eq!(indexer.n_paths(), 131);
    }
}
