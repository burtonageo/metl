
extern crate metl;
extern crate cocoa;

use cocoa::base::{BOOL, nil};
use cocoa::foundation::NSString;
use metl::{CompileOptions, Device, FeatureSet, LanguageVersion, PixelFormat, SpecificLanguageVersion,
           SamplerDescriptor, TextureDescriptor};
use metl::LibraryError;
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
    assert!(!device.name().is_empty());
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

    assert_eq!(command_queue.label(), DUMMY_COMMAND_QUEUE_NAME);
}

#[test]
fn insert_debug_capture_boundary_on_command_queue() {
    let mut device = Device::system_default_device().unwrap();
    let mut command_queue = device.new_command_queue().unwrap();
    command_queue.insert_debug_capture_boundary();
    assert!(true);
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
    println!("{}", device_result.is_ok());
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
    let options =
        CompileOptions::default()
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
#[allow(unused_variables)]
fn create_sampler_and_set_label() {
    const SAMPLER_LABEL: &'static str = "ExampleSampler";
    let mut device = Device::system_default_device().unwrap();

    let mut sampler_descriptor = SamplerDescriptor::new();
    sampler_descriptor.set_label(&SAMPLER_LABEL);
    assert_eq!(sampler_descriptor.label(), SAMPLER_LABEL);

    let sampler = device.new_sampler_state(&sampler_descriptor).unwrap();
    // TODO(burtonageo): Fix getting the label
    // assert_eq!(sampler.label(), sampler_descriptor.label());
}

#[test]
fn create_texture() {
    const PIX_FORMAT: PixelFormat = PixelFormat::Rgba8UnormSrgb;
    const TEX_DIMS: (usize, usize) = (600, 800);
    const MIPPED: bool = false;

    let mut device = Device::system_default_device().unwrap();
    let tex_descriptor = TextureDescriptor::new_2d(PIX_FORMAT, TEX_DIMS.0, TEX_DIMS.1, MIPPED);
    let texture = device.new_texture(&tex_descriptor).unwrap();

    assert_eq!(texture.pixel_format(), PIX_FORMAT);
    assert_eq!(texture.width(), TEX_DIMS.0);
    assert_eq!(texture.height(), TEX_DIMS.1);
    assert_eq!(texture.texture_type(), metl::TextureType::Type2D);
}

#[test]
fn create_invalid_shader() {
    let mut device = Device::system_default_device().unwrap();
    const BAD_SHADER: &'static str = r"abcdefghijklmnopqrstuvwxyz";
    match device.new_library_with_source(&BAD_SHADER, &Default::default()) {
        Ok(_) => panic!("Incorrect result: expected an error"),
        Err(LibraryError::SourceError(_)) => assert!(true),
        Err(_) => panic!("Incorrect error type")
    }
}

#[test]
fn create_invalid_shader_async() {
    let mut device = Device::system_default_device().unwrap();
    const BAD_SHADER: &'static str = r"FooBarBaz";
    match device.new_library_with_source_async(&BAD_SHADER, &Default::default()).recv().unwrap() {
        Ok(_) => panic!("Incorrect result: expected an error"),
        Err(LibraryError::SourceError(_)) => assert!(true),
        Err(_) => panic!("Incorrect error type")
    }
}

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


#[test]
fn device_create_library_with_valid_shader_code_and_get_fn_names() {
    let mut device = Device::system_default_device().unwrap();
    let library = device.new_library_with_source(&SHADER, &Default::default()).ok().unwrap();
    let names = library.function_names();
    assert_eq!(names[0], "vertex_main");
    assert_eq!(names[1], "fragment_main");
}

#[test]
#[ignore]
fn device_create_library_async_with_valid_shader_code_and_get_fn_names() {
    let mut device = Device::system_default_device().unwrap();
    let library = device.new_library_with_source_async(&SHADER, &Default::default())
                        .recv()
                        .unwrap()
                        .unwrap();
    let names = library.function_names();
    assert_eq!(names[0], "vertex_main");
    assert_eq!(names[1], "fragment_main");
}

#[test]
fn shader_get_function_with_name() {
    let mut device = Device::system_default_device().unwrap();
    let mut library = device.new_library_with_source(&SHADER, &Default::default()).ok().unwrap();
    let function = library.new_function_with_name(&"fragment_main");
    assert!(function.is_some());
}

#[test]
#[allow(unused_imports, unused_variables)]
#[ignore]
fn shader_get_struct_info() {
    use metl::{StructMember, StructType};
    let mut device = Device::system_default_device().unwrap();
    let mut library = device.new_library_with_source(&SHADER, &Default::default()).ok().unwrap();
    let function = library.new_function_with_name(&"fragment_main").unwrap();
    unimplemented!();
}
