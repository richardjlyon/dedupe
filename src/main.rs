use dedupe::image::Image;
use exif::Reader;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::BufReader;
use std::path::{Component, PathBuf};
use walkdir::WalkDir;

fn main() {
    // let root = "/Volumes/home/Photos/MobileBackup";
    let root = "/Users/richardlyon/Dev/rust/dedupe/images";
    let mut filepaths: Vec<PathBuf> = Vec::new();
    let mut backup_images: Vec<Image> = Vec::new();
    let mut library_images: Vec<Image> = Vec::new();

    // generate recursive list of images
    println!("Getting images from {} ...", root);
    for entry in WalkDir::new(root) {
        // get image files, assumed to be a result with an extension
        let entry = entry.unwrap();
        let filepath = entry.into_path();
        if filepath.extension().is_some() {
            filepaths.push(filepath)
        }
    }

    // get exif data for 'backup' and 'library' images
    // let number: u64 = filepaths.len() as u64;
    // let bar = ProgressBar::new(number);
    // for filepath in filepaths {
    //     let file = File::open(&filepath).unwrap();
    //     let exif = Reader::new().read_from_container(&mut BufReader::new(&file));
    //     if exif.is_ok() {
    //         // get the library it belongs to, which is the 3rd last path component
    //         let library = match filepath.components().nth_back(3).unwrap() {
    //             Component::Normal(lib) => lib,
    //             _ => continue,
    //         };
    //         // store it in the corresponding vector
    //         match library.to_str().unwrap() {
    //             "MobileBackup" => backup_images.push(Image::new(filepath, exif.unwrap())),
    //             "PhotoLibrary" => library_images.push(Image::new(filepath, exif.unwrap())),
    //             _ => {}
    //         }

    //         bar.inc(1);
    //     }
    //     bar.finish();

    //     println!("Found {} in backup", backup_images.len());
    //     println!("Found {} in library", library_images.len());

    //     // detect duplicates
    //     for library_image in &library_images {
    //         library_image.date_time();
    // }
    // }
}
