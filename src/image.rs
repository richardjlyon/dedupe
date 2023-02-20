use std::path::PathBuf;

use exif::{DateTime, Exif, In, Tag, Value};

pub struct Image {
    filepath: PathBuf,
    exif: Exif,
    is_duplicate: bool,
}

impl Image {
    pub fn new(filepath: PathBuf, exif: Exif) -> Self {
        Self {
            filepath,
            exif,
            is_duplicate: false,
        }
    }

    pub fn date_time(&self) {
        if let Some(field) = self.exif.get_field(Tag::DateTime, In::PRIMARY) {
            match field.value {
                Value::Ascii(ref vec) if !vec.is_empty() => {
                    if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                        println!("Year of DateTime is {}.", datetime.year);
                    }
                }
                _ => {}
            }
        }
    }
}
