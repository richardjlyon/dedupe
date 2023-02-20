use std::{fs::File, io::BufReader, path::PathBuf, fmt};

use exif::{DateTime, Exif, In, Reader, Tag, Value};

use crate::error::AppError;

pub struct Image {
    filepath: PathBuf,
    exif: Exif,
    is_duplicate: bool,
}

impl Image {
    pub fn new(filepath: PathBuf) -> Result<Self, AppError> {
        let file = File::open(&filepath).unwrap();
        let exif = Reader::new().read_from_container(&mut BufReader::new(&file))?;
        Ok(Self {
            filepath,
            exif,
            is_duplicate: false,
        })
    }

    pub fn date_time(&self) -> Result<DateTime, AppError> {
        let field = self.exif.get_field(Tag::DateTime, In::PRIMARY).unwrap();
        match &field.value {
            Value::Ascii(vec) => Ok(DateTime::from_ascii(&vec[0]).unwrap()),
            _ => Err(AppError::DateTimeError),
        }
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image: {}", self.filepath.display())
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        let date_time1 = self.date_time().unwrap();
        let date_time2 = other.date_time().unwrap();

        (date_time1.second == date_time2.second) & 
        (date_time1.minute == date_time2.minute) &
        (date_time1.hour == date_time2.hour) &
        (date_time1.day == date_time2.day) &
        (date_time1.month == date_time2.month) &
        (date_time1.year == date_time2.year)

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
    fn it_computes_equality() {
        let image1_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073239.HEIC");
        let image2_path = PathBuf::from("/Users/richardlyon/Dev/rust/dedupe/images/MobileBackup/2021/06/IMG_20210601_073253.HEIC");

        let image1 = Image::new(image1_path.clone()).unwrap();
        let image2 = Image::new(image2_path.clone()).unwrap();

        assert_eq!(image1, image1);
        assert_ne!(image1, image2);


    }
}
