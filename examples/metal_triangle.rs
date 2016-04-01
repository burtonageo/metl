extern crate metl;

use metl::extras::window::{WindowBuilder};

fn main() {
    let window = WindowBuilder::new().build().unwrap();
    let device = metl::Device::system_default_device().unwrap();

    // todo(burtonageo): implement
}
