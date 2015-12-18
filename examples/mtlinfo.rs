//! A simple example which prints out information about available Metal devices

extern crate mtl;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    println!("Metal is not supported on this platform");
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    let device = match mtl::Device::system_default_device() {
        Ok(device) => device,
        Err(e) => {
            use std::error::Error;
            println!("{}", e.description());
            return;
        }
    };

    println!("Device:\t\t\t\t\t\t{}", device.get_name());
    println!("Supports 24 bit stencil and 8 bit depth:\t{}",
             device.is_depth24_stencil8_pixel_format_supported());
    {
        let max_tpg = device.max_threads_per_group();
        println!("Maximum threads per group:\t\t\t({}, {}, {})",
                 max_tpg.width,
                 max_tpg.height,
                 max_tpg.depth);
    }
    println!("Is low power:\t\t\t\t\t{}", device.is_low_power());
    println!("Is headless:\t\t\t\t\t{}", device.is_headless());
}
