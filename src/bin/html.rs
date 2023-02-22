use maud::{html, Markup, DOCTYPE};
use std::fs::File;
use std::io::{Error, Write};

struct ImageData {
    name: String,
    filepath: String,
}

struct PairData {
    left: ImageData,
    right: ImageData,
}

fn main() -> Result<(), Error> {
    let path = "images.html";
    let mut output = File::create(path)?;

    fn image_row(left: &ImageData, right: &ImageData) -> Markup {
        html! {
                tr {
                    td {(left.name)}
                    td {(right.name)}
                }
                tr {
                    td {img src=(left.filepath) width="300";}
                    td {img src=(right.filepath) width="300";}
                }
        }
    }

    let mut image_pairs: Vec<PairData> = Vec::new();

    let image_fp = String::from("/Volumes/home/Photos/PhotoLibrary/0001/03/0001_0318_1200.jpg");

    let left = ImageData {
        name: String::from("house 1"),
        filepath: image_fp.clone(),
    };
    let right = ImageData {
        name: String::from("house 1 duplicate"),
        filepath: image_fp.clone(),
    };
    image_pairs.push(PairData { left, right });

    let left = ImageData {
        name: String::from("house 2"),
        filepath: image_fp.clone(),
    };
    let right = ImageData {
        name: String::from("house 2 duplicate"),
        filepath: image_fp.clone(),
    };
    image_pairs.push(PairData { left, right });

    let markup = html! {
        (DOCTYPE)
        head {
            title {"duplicates"};
        }
        body {
            table {
                @for image_pair in &image_pairs {
                (image_row(&image_pair.left, &image_pair.right))
                }
            }
        }
    };

    write!(output, "{}", markup.into_string())
}
