use std::fs;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use pdf_writer::{Content, Filter, Finish, Name, PdfWriter, Rect, Ref};
use image::{ColorType, GenericImageView, ImageFormat};

pub fn generate_pdf_from_images(images: Vec<(Vec<u8>, i32, i32)>) {
    // Start writing.
    let mut writer = PdfWriter::new();

    // Define some indirect reference ids we'll use.
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let image_id = Ref::new(4);
    let content_id = Ref::new(6);
    let image_name = Name(b"Im1");

    // Set up the page tree. For more details see `hello.rs`.
    writer.catalog(catalog_id).pages(page_tree_id);
    writer.pages(page_tree_id).kids([page_id]).count(1);

    // Specify one A4 page and map the image name "Im1" to the id of the
    // embedded image stream.
    let mut page = writer.page(page_id);
    let a4 = Rect::new(0.0, 0.0, width as f32, height as f32);
    page.media_box(a4);
    page.parent(page_tree_id);
    page.contents(content_id);
    page.resources().x_objects().pair(image_name, image_id);
    page.finish();

    // Write the stream for the image we want to embed.
    let mut image = writer.image_xobject(image_id, image_data);
    image.width(width);
    image.height(height);
    image.color_space().device_rgb();
    image.bits_per_component(8);
    image.finish();

    // Size the image at 1pt per pixel.
    let w = width as f32;
    let h = height as f32;

    // Center the image on the page.
    let x = (a4.x2 - w) / 2.0;
    let y = (a4.y2 - h) / 2.0;

    // Place and size the image in a content stream.
    let mut content = Content::new();
    content.save_state();
    content.transform([w, 0.0, 0.0, h, x, y]);
    content.x_object(image_name);
    content.restore_state();
    writer.stream(content_id, &content.finish());

    // Write the thing to a file.
    std::fs::write("documents/document.pdf", writer.finish()).unwrap();
}
