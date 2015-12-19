extern crate mtl;
extern crate cocoa;

use mtl::Device;
use mtl::{FromRaw, FromRawError, IntoRaw};
use cocoa::base::nil;
use cocoa::foundation::NSString;

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
    let mut device = Device::system_default_device().unwrap();
    let mut command_queue = device.new_command_queue().unwrap();

    let dummy_command_queue_name = "foo";
    command_queue.set_label(dummy_command_queue_name);

    assert_eq!(command_queue.get_label().into_owned(),
               dummy_command_queue_name.to_string());
}

#[test]
fn insert_debug_capture_boundary_on_command_queue() {
    let mut device = Device::system_default_device().unwrap();
    let mut command_queue = device.new_command_queue().unwrap();
    command_queue.insert_debug_capture_boundary();

    // if nothing panics/fails here, everything is okay
}

#[test]
fn device_from_to_raw() {
    let device = Device::system_default_device().unwrap();
    let raw = device.into_raw();
    let device = Device::from_raw(raw);
    assert!(device.is_ok());
}

#[test]
fn device_from_wrong_type() {
    let some_string = unsafe { NSString::alloc(nil).init_str("Hello") };
    let device_result = Device::from_raw(some_string);
    match device_result {
        Result::Err(FromRawError::WrongPointerType) => {
            // pass
        }
        _ => {
            assert!(false)
        }
    }
}

#[test]
fn device_from_nullptr() {
    let device_result = Device::from_raw(nil);
    match device_result {
        Result::Err(FromRawError::NilPointer) => {
            // pass
        }
        _ => {
            assert!(false)
        }
    }
}

#[test]
fn test_device_create_library_with_valid_shader_code() {
    let mut device = Device::system_default_device().unwrap();
    let shader = r"
        using namespace metal;

        struct ColoredVertex
        {
            float4 position [[position]];
            float4 color;
        };

        vertex ColoredVertex vertex_main(constant float4 *position [[buffer(0)]],
                                         constant float4 *color [[buffer(1)]],
                                         uint vid [[vertex_id]])
        {
            ColoredVertex vert;
            vert.position = position[vid];
            vert.color = color[vid];
            return vert;
        }

        fragment float4 fragment_main(ColoredVertex vert [[stage_in]])
        {
            return vert.color;
        }
    ";
    let library = device.new_library_with_source(shader, &Default::default());
    assert!(library.is_ok());
}
