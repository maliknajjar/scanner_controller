mod scanner;
mod pdf;

fn main() {
    let (image_data, width, height) = scanner::scanner_orders::scan_image();

    // Read the contents of the PPM file into a byte buffer
    let image_data: Vec<u8> = vec![
        255,   0,    0,
          0, 255,    0,
          0,   0,  255,
        255,   0,    0,
    ];
    let height: i32 = 2;
    let width: i32 = 2;

    let images: Vec<(Vec<u8>, i32, i32)> = vec![
        (image_data, width, height),
    ];

    let pdf = pdf::pdf::generate_pdf_from_images(images);
}
