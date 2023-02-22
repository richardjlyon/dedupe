use dedupe::html::{Duplicates, PairData};
use dedupe::indexer::Indexer;
use dedupe::{html::ImageData, image::Image};
use indicatif::ProgressBar;
use maud::{html, DOCTYPE};

fn main() {
    // let root = "/Volumes/home/Photos";
    let root = "/Users/richardlyon/Dev/rust/dedupe/images";
    let mut mobile_indexer = Indexer::new(format!("{}/MobileBackup", root)).unwrap();
    let mut library_indexer = Indexer::new(format!("{}/PhotoLibrary", root)).unwrap();
    let mut mobile_images: Vec<Image> = Vec::new();
    let mut library_images: Vec<Image> = Vec::new();

    // generate list of image file paths
    mobile_indexer.walk();
    library_indexer.walk();

    // get 'mobile' images
    println!("Getting mobile images");
    let bar = ProgressBar::new(mobile_indexer.n_paths());
    for fp in mobile_indexer.filepaths {
        match Image::new(fp) {
            Ok(image) => mobile_images.push(image),
            Err(_) => {}
        }
        bar.inc(1);
    }
    bar.finish();

    // get 'library' images
    println!("Getting library images");
    let bar = ProgressBar::new(library_indexer.n_paths());
    for fp in library_indexer.filepaths {
        match Image::new(fp) {
            Ok(image) => library_images.push(image),
            Err(_) => {}
        }
        bar.inc(1);
    }
    bar.finish();

    // detect duplicates
    let mut duplicates: Duplicates = Duplicates::new();
    for mobile_image in &mobile_images {
        for library_image in &library_images {
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
    println!("{:#?}", duplicates.data);
    duplicates.save_to_disk();
}
