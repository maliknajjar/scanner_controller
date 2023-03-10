use crate::scanner_handler;
use pdf_writer::{Content, Finish, Name, PdfWriter, Rect, Ref};

pub fn generate_pdf_from_images(images: Vec<scanner_handler::Image>) -> Vec<u8> {
    // Start writing.
    let mut writer = PdfWriter::new();

    // Define some indirect reference ids we'll use.
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let mut page_ids = Vec::new();
    let mut id: i32 = 3;
    for image in &images {
        let page_id_number = id;
        let image_id_number = id + 1;
        let content_id_number = id + 2;
        let image_name_number = id + 3;
        let image_name = format!("Image{}", image_name_number);
        let image_name_bytes = image_name.as_bytes();

        let page_id = Ref::new(page_id_number);
        let image_id = Ref::new(image_id_number);
        let content_id = Ref::new(content_id_number);
        let image_name = Name(image_name_bytes);
        id = id + 4;
        page_ids.push(page_id);
    
        // Specify one A4 page and map the image name "Im1" to the id of the
        // embedded image stream.
        let mut page = writer.page(page_id);
        let a4 = Rect::new(0.0, 0.0, image.width as f32, image.height as f32);
        page.media_box(a4);
        page.parent(page_tree_id);
        page.contents(content_id);
        page.resources().x_objects().pair(image_name, image_id);
        page.finish();
    
        // Write the stream for the image we want to embed.
        let mut image_stream = writer.image_xobject(image_id, &image.data);
        image_stream.width(image.width);
        image_stream.height(image.height);
        image_stream.color_space().device_rgb();
        image_stream.bits_per_component(8);
        image_stream.finish();

        // Size the image at 1pt per pixel.
        let w = image.width as f32;
        let h = image.height as f32;
    
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
    }

    // Set up the page tree. For more details see `hello.rs`.
    writer.catalog(catalog_id).pages(page_tree_id);
    writer.pages(page_tree_id).kids(page_ids).count(images.len() as i32);

    return writer.finish();
}
