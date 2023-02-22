/*!
 * Code to render pairs of images in a simple html file to allow visual comparison of results.
 */

use crate::error::AppError;
use maud::{html, Markup, DOCTYPE};
use std::{fs::File, io::Write, path::PathBuf};

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

pub fn to_html(duplicates: &Duplicates) -> Result<(), AppError> {
    let path = "images.html";
    let mut output = File::create(path)?;

    fn image_row(left: &ImageData, right: &ImageData) -> Markup {
        html! {
                tr {
                    td {(left.name)}
                    td {(right.name)}
                }
                tr {
                    td {img src=(left.filepath.to_str().unwrap()) width="300";}
                    td {img src=(right.filepath.to_str().unwrap()) width="300";}
                }
        }
    }

    let markup = html! {
        (DOCTYPE)
        head {
            title {"duplicates"};
        }
        body {
            table {
                @for image_pair in &duplicates.data {
                (image_row(&image_pair.left_image, &image_pair.right_image))
                }
            }
        }
    };

    write!(output, "{}", markup.into_string());

    Ok(())
}
