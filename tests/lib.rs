extern crate metl;
extern crate cocoa;

use cocoa::base::{BOOL, nil};
use cocoa::foundation::NSString;
use metl::{CompileOptions, Device, LanguageVersion, SpecificLanguageVersion, FeatureSet};
use metl::LibraryErrorType;
use metl::{FromRaw, FromRawError, IntoRaw};
use metl::sys::{MTLCompileOptions, MTLLanguageVersion};

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

#[cfg(target_os = "macos")]
#[test]
fn device_supports_base_features() {
    let base_osx_feature_set = FeatureSet::OsxGpuFamily1_v1;
    let device = Device::system_default_device().unwrap();
    assert!(device.supports_feature_set(base_osx_feature_set));
}

#[test]
fn set_get_command_queue_label() {
    let mut device = Device::system_default_device().unwrap();
    let mut command_queue = device.new_command_queue().unwrap();

    const DUMMY_COMMAND_QUEUE_NAME: &'static str = "foo";
    command_queue.set_label(&DUMMY_COMMAND_QUEUE_NAME);

    assert_eq!(command_queue.label(),
               DUMMY_COMMAND_QUEUE_NAME);
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
fn compile_opts_creation_is_correct() {
    let fast_math_enabled = true;
    let options = CompileOptions::default()
                      .fast_math_enabled(fast_math_enabled)
                      .language_version(LanguageVersion::Specific(SpecificLanguageVersion::Version_1_0))
                      .with_macro("Foo", 54)
                      .with_macro("Bar", 32.0)
                      .with_macro("Baz", "Hi")
                      .mtl_compile_options();

    unsafe {
        assert_eq!(fast_math_enabled as BOOL, options.fastMathEnabled());
        assert_eq!(options.languageVersion(),
                   MTLLanguageVersion::MTLLanguageVersion1_0);
    }
}

#[test]
fn create_invalid_shader() {
    let mut device = Device::system_default_device().unwrap();
    const BAD_SHADER: &'static str = r"abcdefghijklmnopqrstuvwxyz";
    match device.new_library_with_source(&BAD_SHADER, &Default::default()) {
        Ok(_) => panic!("Incorrect result: expected an error"),
        Err(error) => {
            if let LibraryErrorType::SourceError = error.error_type {
                assert!(true);
            } else {
                panic!("Incorrect error type");
            }
        }
    }
}

#[test]
fn device_create_library_with_valid_shader_code_and_get_fn_names() {
    let mut device = Device::system_default_device().unwrap();
    // Shader source taken from http://metalbyexample.com/up-and-running-2/
    const SHADER: &'static str = r"
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
    let library = device.new_library_with_source(&SHADER, &Default::default()).ok().unwrap();
    let names = library.function_names();
    assert_eq!(names[0], "vertex_main");
    assert_eq!(names[1], "fragment_main");
}
