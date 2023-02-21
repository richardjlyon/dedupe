//! Functionality for representing an image and obtaining creation date, modified date, and dimensions.
//!
//!

use std::{fmt, fs::metadata, fs::File, hash::Hasher, io::BufReader, path::PathBuf};

use exif::{DateTime, Exif, In, Reader, Tag, Value};

use chrono::prelude::*;
use filetime::FileTime;
use highway::{HighwayHash, PortableHash};
use crate::error::AppError;
use log;

/// Represents the dimensions of the image as [width x height].
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dimensions: {}x{}", self.width, self.height)
    }
}

/// Represents an image.
pub struct Image {
    pub filepath: PathBuf,
    exif: Exif,
    modified_time: chrono::DateTime<Utc>,
    is_duplicate: bool,
}

impl Image {
    /// Construct a new image from its file path.
    pub fn new(filepath: PathBuf) -> Result<Self, AppError> {
        let file = File::open(&filepath).unwrap();
        let exif = Reader::new().read_from_container(&mut BufReader::new(&file))?;

        let metadata = metadata(&filepath).unwrap();
        let file_time = FileTime::from_last_modification_time(&metadata);
        let unix_seconds = file_time.unix_seconds();
        let naive = NaiveDateTime::from_timestamp_opt(unix_seconds, 0).unwrap();
        let modified_time: chrono::DateTime<Utc> = chrono::DateTime::from_utc(naive, Utc);

        Ok(Self {
            filepath,
            exif,
            modified_time,
            is_duplicate: false,
        })
    }

    /// Get the image creation date.
    pub fn date_time(&self) -> Result<DateTime, AppError> {
        // let field = self.exif.get_field(Tag::DateTime, In::PRIMARY).unwrap();
        let field = match self.exif.get_field(Tag::DateTime, In::PRIMARY) {
            Some(data) => data,
            None => return Err(AppError::DateTimeError),
        };
        match &field.value {
            Value::Ascii(vec) => Ok(DateTime::from_ascii(&vec[0]).unwrap()),
            _ => Err(AppError::DateTimeError),
        }
    }

    /// Get the image pixel dimensions.
    pub fn pixel_dimension(&self) -> Result<Dimensions, AppError> {
        let width = match self.exif.get_field(Tag::PixelXDimension, In::PRIMARY) {
            Some(field) => {
                if let Some(width) = field.value.get_uint(0) {
                    width
                } else {
                    return Err(AppError::DimensionError);
                }
            }
            None => return Err(AppError::DimensionError),
        };

        let height = match self.exif.get_field(Tag::PixelYDimension, In::PRIMARY) {
            Some(field) => {
                if let Some(height) = field.value.get_uint(0) {
                    height
                } else {
                    return Err(AppError::DimensionError);
                }
            }
            None => return Err(AppError::DimensionError),
        };

        Ok(Dimensions { width, height })
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image: {}", self.filepath.display())
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        // returns false if either image has no datetime
        if self.date_time().is_err()
            || other.date_time().is_err()
            || self.pixel_dimension().is_err()
            || other.pixel_dimension().is_err()
        {
            log::warn!("This is an example message.");
            return false;
        }

        let date_time1 = self.date_time().unwrap();
        let date_time2 = other.date_time().unwrap();

        let dimensions1 = self.pixel_dimension().unwrap();
        let dimensions2 = other.pixel_dimension().unwrap();

        // check exif parameters
        let exif_match = (date_time1.second == date_time2.second)
            & (date_time1.minute == date_time2.minute)
            & (date_time1.hour == date_time2.hour)
            & (date_time1.day == date_time2.day)
            & (date_time1.month == date_time2.month)
            & (date_time1.year == date_time2.year)
            & (dimensions1.height == dimensions1.height)
            & (dimensions2.height == dimensions2.height);

        // check hash
        if exif_match {
            let mut file1 = File::open(&self.filepath).unwrap();
            let mut file2 = File::open(&other.filepath).unwrap();
            let mut hasher1 = PortableHash::default();
            let mut hasher2 = PortableHash::default();

            std::io::copy(&mut file1, &mut hasher1).unwrap();
            std::io::copy(&mut file2, &mut hasher2).unwrap();

            let hash64_1 = hasher1.finalize256();
            let hash64_2 = hasher2.finalize256();

            return hash64_1 == hash64_2;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let test_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073239.HEIC");
        let image = Image::new(test_path.clone()).unwrap();
        assert_eq!(image.filepath, test_path);
        assert_eq!(image.is_duplicate, false);
        println!("{:?}", image.modified_time);
    }

    #[test]
    fn it_gets_date_time() {
        // expect created date to be DateTime { year: 2021, month: 6, day: 1, hour: 7, minute: 32, second: 39, nanosecond: None, offset: None }
        let test_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073239.HEIC");
        let image = Image::new(test_path.clone()).unwrap();
        let date_time = image.date_time().unwrap();

        assert_eq!(date_time.year, 2021);
        assert_eq!(date_time.month, 6);
        assert_eq!(date_time.day, 1);
        assert_eq!(date_time.hour, 7);
        assert_eq!(date_time.minute, 32);
        assert_eq!(date_time.second, 39);
    }

    #[test]
    fn it_gets_pixel_dimension() {
        let test_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073239.HEIC");
        let image = Image::new(test_path.clone()).unwrap();

        assert_eq!(image.pixel_dimension().unwrap().width, 4032);
        assert_eq!(image.pixel_dimension().unwrap().height, 3024);
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
