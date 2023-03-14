#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::stream::{Event, EventStream};
use rocket::tokio::time::{self, Duration};

#[get("/sse")]
fn sse() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            yield Event::data("ping");
            interval.tick().await;
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![sse, index])
}


/////////////////////////////////////////////////////////////
// example of the hash and scanner and pdf functions usage //
/////////////////////////////////////////////////////////////
// // scan two times and get the image struct
// let mut images: Vec<scanner_handler::Image> = Vec::new();

// println!("starting scanning");
// images.push(scanner_handler::scan_image());
// images.push(scanner_handler::scan_image());

// // two simple pixels
// println!("starting..");
// images.push(scanner_handler::Image { data: vec![ 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0 ], height: 2, width: 2, hash: None });
// images.push(scanner_handler::Image { data: vec![ 0, 255, 0, 255, 0, 0, 0, 0, 255, 0, 255, 0 ], height: 2, width: 2, hash: None });

// // hash every image data (create a function that does this)
// hash_handler::generate_and_fill_images_hashes(&mut images);

// // generate pdf file from the images array
// let pdf_data = pdf_handler::generate_pdf_from_images(images);

// // Write the thing to a file.
// std::fs::write("documents/documents.pdf", pdf_data).expect("Failed writing the pdf file");