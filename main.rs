use sane_scan::Sane;

fn main() {
    let sane = Sane::init(0).unwrap();
    let devices = sane.get_devices().unwrap();
    if devices.is_empty() {
        println!("Error: No scanners found.");
        return;
    }
}
