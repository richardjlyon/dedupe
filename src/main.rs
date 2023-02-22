use dedupe::html::{to_html, Duplicates, PairData};
use dedupe::image::Images;
use dedupe::indexer::Indexer;
use dedupe::{html::ImageData, image::Image};
use indicatif::ProgressBar;
use maud::{html, DOCTYPE};
use std::fs;

fn main() {
    let from_disk = true;
    let mut mobile_images: Images = Images::new();
    let mut library_images: Images = Images::new();

    if from_disk {
        let mobile_json =
            fs::read_to_string("mobile.json").expect("Should have been able to read mobile json");
        let library_json =
            fs::read_to_string("library.json").expect("Should have been able to read library json");

        mobile_images = serde_json::from_str(&mobile_json).unwrap();
        library_images = serde_json::from_str(&library_json).unwrap();
    } else {
        // let root = "/Volumes/home/Photos";
        let root = "/Users/richardlyon/Dev/rust/dedupe/images";
        let mut mobile_indexer = Indexer::new(format!("{}/MobileBackup", root)).unwrap();
        let mut library_indexer = Indexer::new(format!("{}/PhotoLibrary", root)).unwrap();

        // generate list of image file paths
        mobile_indexer.walk();
        library_indexer.walk();

        // get 'mobile' images
        println!("Getting mobile images");
        let bar = ProgressBar::new(mobile_indexer.n_paths());
        for fp in mobile_indexer.filepaths {
            match Image::new(fp) {
                Ok(image) => mobile_images.data.push(image),
                Err(_) => {}
            }
            bar.inc(1);
        }
        bar.finish();
        mobile_images.save_to_disk("mobile");

        // get 'library' images
        println!("Getting library images");
        let bar = ProgressBar::new(library_indexer.n_paths());
        for fp in library_indexer.filepaths {
            match Image::new(fp) {
                Ok(image) => library_images.data.push(image),
                Err(_) => {}
            }
            bar.inc(1);
        }
        bar.finish();
        library_images.save_to_disk("library");
    }

    // detect duplicates
    let mut duplicates: Duplicates = Duplicates::new();
    for mobile_image in &mobile_images.data {
        for library_image in &library_images.data {
            if mobile_image == library_image {
                let left_image = ImageData {
                    // TODO find a better way that avoinds clone()
                    name: mobile_image.clone().file_name(),
                    filepath: mobile_image.filepath.clone(),
                };
                let right_image = ImageData {
                    // TODO find a better way that avoinds clone()
                    name: library_image.clone().file_name(),
                    filepath: library_image.filepath.clone(),
                };
                duplicates.data.push(PairData {
                    left_image,
                    right_image,
                });
            }
        }
    }

    // visualise

    to_html(&duplicates);
}
