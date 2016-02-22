//! A simple example which prints out information about available Metal devices

extern crate mtl;


#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    println!("Metal is not supported on this platform");
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    let devices = mtl::Device::enumerate_all_system_devices();

    let num_devices_message = format!("Number of system devices: {}", devices.len());
    let separator = std::iter::repeat('=')
                        .take(num_devices_message.len())
                        .collect::<String>();

    println!("{}", num_devices_message);
    println!("{}", separator);
    println!("");

    for (index, device) in devices.into_iter().enumerate() {
        println!("Device {} info:", index);
        let device = match device {
            Ok(device) => device,
            Err(e) => {
                use std::error::Error;
                println!("{}", e.description());
                continue
            }
        };

        println!("Name:\t\t\t\t\t\t{}", device.name());
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
        println!("");
    }
}
