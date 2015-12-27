extern crate mtl;
extern crate cocoa;

use mtl::{CompileOptions, Device, LanguageVersion, SpecificLanguageVersion};
use mtl::LibraryError;
use mtl::{FromRaw, FromRawError, IntoRaw};
use mtl::sys::{MTLCompileOptions, MTLLanguageVersion};
use cocoa::base::{BOOL, nil};
use cocoa::foundation::NSString;

#[test]
fn get_device() {
    let device = Device::system_default_device();
    assert!(device.is_ok());
}

#[test]
fn get_all_devices() {
    let all_devices = Device::enumerate_all_system_devices();
    assert!(!all_devices.is_empty());
    for device in all_devices {
        assert!(device.is_ok());
    }
}

#[test]
fn get_device_name() {
    let device = Device::system_default_device().unwrap();
    assert!(!device.name().into_owned().is_empty());
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
        _ => assert!(false),
    }
}

#[test]
fn device_from_nullptr() {
    let device_result = Device::from_raw(nil);
    match device_result {
        Result::Err(FromRawError::NilPointer) => {
            // pass
        }
        _ => assert!(false),
    }
}

#[test]
fn test_compile_opts_creation_is_correct() {
    let mut options = CompileOptions::default();

    let fast_math_enabled = true;
    options.fast_math_enabled = Some(fast_math_enabled);
    options.language_version = LanguageVersion::Specific(SpecificLanguageVersion::Version_1_0);

    let native_options = options.mtl_compile_options();

    unsafe {
        assert_eq!(fast_math_enabled as BOOL,
                   native_options.fastMathEnabled());
        assert_eq!(native_options.languageVersion(),
                   MTLLanguageVersion::MTLLanguageVersion1_0);
    }
}

#[test]
fn create_invalid_shader() {
    let mut device = Device::system_default_device().unwrap();
    let bad_shader = r"abcdefghijklmnopqrstuvwxyz";
    match device.new_library_with_source(bad_shader, &Default::default()) {
        Ok(_) => panic!("Incorrect result: expected an error"),
        Err(LibraryError::SourceError) => assert!(true),
        _ => panic!("Incorrect result: unexpected error"),
    }
}

#[test]
fn test_device_create_library_with_valid_shader_code() {
    let mut device = Device::system_default_device().unwrap();
    // Shader source taken from http://metalbyexample.com/up-and-running-2/
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
