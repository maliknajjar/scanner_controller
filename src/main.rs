mod scanner;
mod pdf;

use scanner::scanner_handler::{self, Image};
use pdf::pdf_handler;

fn main() {
    // let image: scanner_handler::Image = scanner_handler::scan_image();

    // Read the contents of the PPM file into a byte buffer
    let images: Vec<scanner_handler::Image> = vec![
        Image {
            data: vec![
                255,   0,    0,
                  0, 255,    0,
                  0,   0,  255,
                255,   0,    0,
            ],
            height: 2,
            width: 2
        },
    ];

    let pdf = pdf_handler::generate_pdf_from_images(images);
}
