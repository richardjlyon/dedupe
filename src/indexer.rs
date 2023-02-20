//! Functionality for indexing a folder of images.
//!
//!

use std::path::PathBuf;

use walkdir::WalkDir;

use crate::error::AppError;

pub struct Indexer {
    root: String,
}

impl Indexer {
    /// Construct a new indexer
    pub fn new(root: &str) -> Result<Self, AppError> {
        if !std::path::Path::new(&root).exists() {
            return Err(AppError::NetworkError);
        }
        Ok(Self {
            root: String::from(root),
        })
    }

    /// Walk the given root and populate filepaths
    pub fn walk(self, filepaths: &mut Vec<PathBuf>) {
        for entry in WalkDir::new(self.root) {
            // get image files, assumed to be a result with an extension
            let entry = entry.unwrap();
            let filepath = entry.into_path();
            if filepath.extension().is_some() {
                filepaths.push(filepath)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let good_root = "/Users/richardlyon/Dev/rust/dedupe/images";
        let indexer = Indexer::new(good_root);
        assert!(indexer.is_ok());

        let bad_root = "/BadRoot";
        let indexer = Indexer::new(bad_root);
        assert!(indexer.is_err());
    }

    #[test]
    fn it_walks() {
        let root = "/Users/richardlyon/Dev/rust/dedupe/images";
        let indexer = Indexer::new(root).unwrap();
        let mut filepaths: Vec<PathBuf> = Vec::new();

        indexer.walk(&mut filepaths);

        assert_eq!(filepaths.len(), 131);
    }
}
