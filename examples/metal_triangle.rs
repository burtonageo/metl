extern crate metl;

use metl::extras::window::WindowBuilder;

fn main() {
    let window = WindowBuilder::new().build().unwrap();
    let _device = metl::Device::system_default_device().unwrap();

    for _event in window.events() {
        // do nothing
    }
}
