extern crate metl;

use metl::extras::window::WindowBuilder;

fn main() {
    let window = WindowBuilder::new().build().unwrap();
    let _device = metl::Device::system_default_device().unwrap();

    for event in window.poll_events() {
        println!("{:?}", event);
    }
}
