mod scanner;
mod pdf;

use scanner::scanner_handler::{self, Image};
use pdf::pdf_handler;

fn main() {
    // Read the contents of the PPM file into a byte buffer
    let mut images: Vec<scanner_handler::Image> = Vec::new();
    images.push(scanner_handler::scan_image());
    images.push(scanner_handler::scan_image());

    let pdf_data = pdf_handler::generate_pdf_from_images(images);
    // Write the thing to a file.
    std::fs::write("documents/document.pdf", pdf_data).expect("Failed generating a pdf file");
}
