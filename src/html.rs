/*!
 * Code to render pairs of images in a simple html file to allow visual comparison of results.
 */

use std::{fs::File, io::Write, path::PathBuf};

use crate::error::AppError;

#[derive(Debug)]
pub struct ImageData {
    pub name: String,
    pub filepath: PathBuf,
}

#[derive(Debug)]
pub struct PairData {
    pub left_image: ImageData,
    pub right_image: ImageData,
}

#[derive(Debug)]
pub struct Duplicates {
    pub data: Vec<PairData>,
}

impl Duplicates {
    pub fn new() -> Self {
        let data: Vec<PairData> = Vec::new();
        Self { data }
    }
}
