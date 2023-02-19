use walkdir::{WalkDir, DirEntry};
use exif::{DateTime, In, Reader, Value, Tag, Exif};
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use indicatif::ProgressBar;

struct Image {
    filepath: PathBuf,
    exif: Exif,
}

fn main() {

    // let root = "/Volumes/home/Photos/MobileBackup";
    let root = "/Users/richardlyon/Dev/rust/dedupe/images";
    let mut filepaths:Vec<PathBuf> = Vec::new();
    let mut images:Vec<Image> = Vec::new();  
    
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
    
    // get exif data
    let number:u64 =  filepaths.len() as u64;
    let bar = ProgressBar::new(number);
    for filepath in filepaths {
        let file = File::open(&filepath).unwrap();
        let exif = Reader::new()
            .read_from_container(&mut BufReader::new(&file));
        if exif.is_ok() {
            images.push(Image{filepath:filepath, exif:exif.unwrap()})
        }
        bar.inc(1);
    }
    bar.finish()

    //         Err(_) => continue,
    //         Ok(exif) => images.push(Image{filepath: filepath, exif: exif})
    //     };
    //     print!(".");
    // }


    // parse exif data
    // println!("Analysing images...");
    // for image in images {

    //     let file = File::open(&image.filepath).unwrap();

    //     let exif = match Reader::new()
    //         .read_from_container(&mut BufReader::new(&file)) {
    //         Err(_) => continue,
    //         Ok(e) => e
    //     };
            
    //     if let Some(field) = exif.get_field(Tag::PixelXDimension, In::PRIMARY){
    //             if let Some(width) = field.value.get_uint(0) {
    //                     println!("Valid width of the image is {}.", width);
    //                 }
    //             }
    //         }
    

    // detect and mark duplicates for deletion

    // delete duplicates

}
