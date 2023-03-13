mod scanner;
mod pdf;

use scanner::scanner_handler::{self, Image};
use pdf::pdf_handler;
use sha2::{Sha256, Digest};

fn main() {
    // // scan two times and get the image struct
    // let mut images: Vec<scanner_handler::Image> = Vec::new();
    // images.push(scanner_handler::scan_image());
    // images.push(scanner_handler::scan_image());

    // creating two simple images vector
    let mut images: Vec<scanner_handler::Image> = vec![
        Image {
            data: vec![
                255, 0, 0,
                0, 255, 0,
                0, 0, 255,
                255, 0, 0,
            ],
            height: 2,
            width: 2,
            hash: None
        },
        Image {
            data: vec![
                0, 255, 0,
                255, 0, 0,
                0, 0, 255,
                0, 255, 0,
            ],
            height: 2,
            width: 2,
            hash: None
        },
    ];

    // hash every image data
    for image in &mut images {
        let mut hasher = Sha256::new();
        hasher.update(&image.data);
        let hash_string = format!("{:x}", hasher.finalize());
        image.hash = Some(hash_string.clone());
        println!("hash: {}", hash_string);
        println!("");
    }

    let pdf_data = pdf_handler::generate_pdf_from_images(images);
    // Write the thing to a file.
    std::fs::write("documents/document.pdf", pdf_data).expect("Failed generating a pdf file");
}
 