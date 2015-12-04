//! A simple example which prints out information about available Metal devices

extern crate mtl;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    println!("Metal is not supported on this platform");
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    use mtl::ReadOnlyDevice;
    let device = match mtl::Device::system_default_device() {
        Ok(device) => device,
        Err(e) => {
            use std::error::Error;
            println!("{}", e.description());
            return;
        }
    };

    println!("Device: {}", device.get_name());
    println!("Supports 24 bit stencil, 8 bit depth: {}", device.is_depth24_stencil8_pixel_format_supported());
    println!("Is low power: {}", device.is_low_power());
    println!("Is headless: {}", device.is_headless());
}
