use std::process::Command;

fn main() {
    // Install build essentials
    let output = Command::new("sudo")
                         .arg("apt-get")
                         .arg("install")
                         .arg("build-essential")
                         .output()
                         .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Install libclang-dev
    let output = Command::new("sudo")
                         .arg("apt-get")
                         .arg("install")
                         .arg("libclang-dev")
                         .output()
                         .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Install libsane-dev
    let output = Command::new("sudo")
                         .arg("apt-get")
                         .arg("install")
                         .arg("libsane-dev")
                         .output()
                         .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
