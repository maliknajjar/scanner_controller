use crate::scanner_handler;
use sha2::{Sha256, Digest};

pub fn generate_and_fill_images_hashes(images: &mut Vec<scanner_handler::Image>) {
    for image in images {
        image.hash = Some(generate_hash_from_bytes(&image.data));
    }
}

pub fn generate_hash_from_bytes(bytes: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash_string = format!("{:x}", hasher.finalize());
    return hash_string;
}