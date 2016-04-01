extern crate metl;

use metl::Device;
use metl::extras::window::{WindowBuilder};

fn main() {
    let window = WindowBuilder::new().build().unwrap();
    let device = Device::system_default_device().unwrap();

    // todo(burtonageo): implement
}
