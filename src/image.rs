/*!
Functionality for representing an image and obtaining creation date, modified date, and dimensions.
*/

use std::{fmt, fs::File, io::Write, path::PathBuf};

use crate::error::AppError;
use highway::{HighwayHash, PortableHash};
use serde::{Deserialize, Serialize};

/// Represents an image.
#[derive(Serialize, Deserialize, Clone)]
pub struct Image {
    pub filepath: PathBuf,
    pub hash64: [u64; 4],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub data: Vec<Image>,
}

impl Image {
    /// Construct a new image from its file path.
    pub fn new(filepath: PathBuf) -> Result<Self, AppError> {
        let mut file = File::open(&filepath).unwrap();
        let mut hasher = PortableHash::default();
        std::io::copy(&mut file, &mut hasher).unwrap();
        let hash64 = hasher.finalize256();

        Ok(Self { filepath, hash64 })
    }

    // Compute the filename.
    pub fn file_name(self) -> String {
        let filename = self.filepath.file_name().unwrap().to_str().unwrap();
        String::from(filename)
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image: {}", self.filepath.display())
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.hash64 == other.hash64
    }
}

impl Images {
    pub fn new() -> Self {
        let data: Vec<Image> = Vec::new();
        Self { data }
    }

    pub fn save_to_disk(&self, filename: &str) -> Result<(), AppError> {
        let serialised = serde_json::to_string_pretty(&self)?;
        let mut output = File::create(format!("{}.json", filename))?;
        write!(output, "{}", serialised);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_an_image() {
        let test_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073239.HEIC");
        let image = Image::new(test_path.clone()).unwrap();
        assert_eq!(image.filepath, test_path);
    }

    #[test]
    fn it_computes_filename() {
        let test_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073239.HEIC");
        let image = Image::new(test_path.clone()).unwrap();
        assert_eq!(image.file_name(), "IMG_20210601_073239.HEIC");
    }

    #[test]
    fn it_computes_equality() {
        let image1_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073239.HEIC");
        let image2_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073253.HEIC");

        let image1 = Image::new(image1_path.clone()).unwrap();
        let image2 = Image::new(image2_path.clone()).unwrap();

        assert_eq!(image1, image1);
        assert_ne!(image1, image2);
    }
}
