use crate::scanner_handler;
use sha2::{Sha256, Digest};

pub fn fill_images_hashes(images: &mut Vec<scanner_handler::Image>) {
    for image in images {
        let mut hasher = Sha256::new();
        hasher.update(&image.data);
        let hash_string = format!("{:x}", hasher.finalize());
        image.hash = Some(hash_string.clone());
    }
}

pub fn generate_hash_from_bytes(bytes: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash_string = format!("{:x}", hasher.finalize());
    return hash_string;
}