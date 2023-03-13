use sane_scan::Sane;

pub struct Image {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

pub fn scan_image() -> Image {
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
    
    let image = Image { data: image_data, width: width, height: height};

    return image;
}
