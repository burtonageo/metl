extern crate mtl;

use mtl::{CommandQueue, Device};

#[test]
fn get_device() {
    let device = Device::system_default_device();
    assert!(device.is_ok());
}

#[test]
fn get_device_name() {
    let device = Device::system_default_device().unwrap();
    assert!(!device.get_name().into_owned().is_empty());
}

#[test]
fn set_get_command_queue_label() {
    let device = Device::system_default_device().unwrap();
    let command_queue = CommandQueue::new(&device).unwrap();

    let dummy_command_queue_name = "foo";
    command_queue.set_label(dummy_command_queue_name);

    assert_eq!(command_queue.get_label().into_owned(), dummy_command_queue_name.to_string());
}

#[test]
fn insert_debug_capture_boundary_on_command_queue() {
    let device = Device::system_default_device().unwrap();
    let command_queue = CommandQueue::new(&device).unwrap();
    command_queue.insert_debug_capture_boundary();

    // if nothing panics/fails here, everything is okay
}
