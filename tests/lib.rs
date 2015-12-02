extern crate mtl;
extern crate libc;

use mtl::{Device};

#[test]
fn get_device() {
    let device = Device::system_default_device();
    assert!(device.is_ok());
}

#[test]
fn get_device_name() {
    let device = Device::system_default_device().unwrap();
    assert!(!device.name().into_owned().is_empty());
}
