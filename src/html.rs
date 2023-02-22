/*!
 * Code to render pairs of images in a simple html file to allow visual comparison of results.
 */

use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    pub name: String,
    pub filepath: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PairData {
    pub left_image: ImageData,
    pub right_image: ImageData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Duplicates {
    pub data: Vec<PairData>,
}

impl Duplicates {
    pub fn new() -> Self {
        let data: Vec<PairData> = Vec::new();
        Self { data }
    }

    pub fn save_to_disk(&self) -> Result<(), AppError> {
        let serialised = serde_json::to_string_pretty(&self)?;
        let mut output = File::create("pairs.json")?;
        write!(output, "{}", serialised);

        Ok(())
    }
}
