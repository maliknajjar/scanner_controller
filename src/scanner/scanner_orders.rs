use sane_scan::{Sane};

pub fn scan_image() -> (Vec<u8>, i32, i32) {
    let sane = Sane::init_1_0().expect("Failed to initialize SANE");
    let mut devices = sane.get_devices().expect("Failed to enumerate devices");
    let device = devices.first_mut().expect("No scanner found");
    println!("{:?}", device);

    let mut handle = device.open().expect("Failed to open the device handle");

    let options = handle.get_options().expect("Failed to get options");
    for option in options.iter() {
        println!("{:?}", option);
    }

    let params = handle.get_parameters().expect("Failed to get parameters");

    let params = handle.start_scan().expect("Failed to start scan");

    let width = params.pixels_per_line as i32;
    let height = params.lines as i32;
    let image_data = handle.read_to_vec().expect("Failed to read images raw bytes");
    return (image_data, width, height);
}
