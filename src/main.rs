mod scanner;
mod pdf;
mod hash;

use scanner::scanner_handler;
use pdf::pdf_handler;
use hash::hash_handler;

fn main() {
    // scan two times and get the image struct
    let mut images: Vec<scanner_handler::Image> = Vec::new();

    println!("starting scanning");
    images.push(scanner_handler::scan_image());
    images.push(scanner_handler::scan_image());

    // // two simple pixels
    // images.push(Image { data: vec![ 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0 ], height: 2, width: 2, hash: None });
    // images.push(Image { data: vec![ 0, 255, 0, 255, 0, 0, 0, 0, 255, 0, 255, 0 ], height: 2, width: 2, hash: None });

    // hash every image data (create a function that does this)
    hash_handler::fill_images_hashes(&mut images);
    for image in &images {
        println!("hash: {}", image.hash.as_ref().unwrap());
    }

    // generate pdf file from the images array
    let pdf_data = pdf_handler::generate_pdf_from_images(images);

    // Write the thing to a file.
    std::fs::write("documents/documents.pdf", pdf_data).expect("Failed generating a pdf file");
}
 