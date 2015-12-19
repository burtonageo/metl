use cocoa::base::{id, nil};
use cocoa::foundation::{NSString, NSUInteger};
use objc::runtime::YES;
use sys::{MTLCopyAllDevices, MTLCreateSystemDefaultDevice, MTLDevice};
use std::borrow::Cow;
use std::convert::{AsRef, From};
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use {CommandQueue, CommandQueueError, CompileOptions, FromRaw, FromRawError, Library,
     LibraryError, Size};

pub struct Device(id);

impl Device {
    pub fn system_default_device() -> Result<Self, DeviceError> {
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        if device != nil { Ok(Device(device)) } else { Err(DeviceError::ConstructionFailed) }
    }

    #[allow(unused_mut, unused_variables, unreachable_code)]
    pub fn enumerate_all_system_devices() -> Vec<Result<Self, DeviceError>> {
        let all_devices = unsafe { MTLCopyAllDevices() };
        let mut devices_vec = vec![];

        unimplemented!();

        devices_vec
    }

    pub fn new_command_queue(&mut self) -> Result<CommandQueue, CommandQueueError> {
        let command_queue = unsafe { self.0.newCommandQueue() };
        Ok(try!(FromRaw::from_raw(command_queue)))
    }

    pub fn new_command_queue_with_max_buffer_count(&mut self, max_command_buffer_count: usize)
                                                   -> Result<CommandQueue, CommandQueueError> {
        let command_queue = unsafe {
            self.0.newCommandQueueWithMaxCommandBufferCount(max_command_buffer_count as NSUInteger)
        };
        Ok(try!(FromRaw::from_raw(command_queue)))
    }

    #[allow(unused_variables)]
    pub fn new_default_library(&mut self) -> Result<Library, LibraryError> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    pub fn new_library_with_file<P: AsRef<Path>>(&mut self, file_path: P)
                                                 -> Result<Library, LibraryError> {
        unimplemented!();
    }

    pub fn new_library_with_source<S: AsRef<str>>(
        &mut self, source: S, compile_options: &CompileOptions)
        -> Result<Library, LibraryError> {
        unsafe {
            let source = NSString::alloc(nil).init_str(source.as_ref());
            let options = compile_options.mtl_compile_options();
            let error = nil;
            let library = self.0.newLibraryWithSource_options_error(source, options, error);
            if error != nil {
                // Todo(George): Should use the `error` variable to get more info
                Err(LibraryError::SourceError)
            } else {
                Ok(try!(FromRaw::from_raw(library)))
            }
        }
    }

    #[allow(unused_variables)]
    // Todo(George): `data` parameter should be of the correct type
    pub fn new_library_with_data(&mut self, data: ()) -> Result<Library, LibraryError> {
        unimplemented!();
    }

    pub fn is_depth24_stencil8_pixel_format_supported(&self) -> bool {
        unsafe { self.0.depth24Stencil8PixelFormatSupported() == YES }
    }

    pub fn is_headless(&self) -> bool {
        unsafe { self.0.headless() == YES }
    }

    pub fn is_low_power(&self) -> bool {
        unsafe { self.0.lowPower() == YES }
    }

    pub fn max_threads_per_group(&self) -> Size {
        unsafe { self.0.maxThreadsPerGroup().into() }
    }

    pub fn get_name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_string_lossy() }
    }

    pub fn supports_texture_sample_count(&self, sample_count: usize) -> bool {
        unsafe { self.0.supportsTextureSampleCount(sample_count as NSUInteger) == YES }
    }
}

impl_from_into_raw!(Device, "MTLDevice");

/// Internal utility function to get a Device's id without consuming it.
/// Not exported publicly from this crate.
#[doc(hidden)]
pub unsafe fn _device_get_raw(device: &Device) -> id {
    device.0
}

#[derive(Clone, Debug)]
pub enum DeviceError {
    ConstructionFailed,
    RawError(FromRawError)
}

impl From<FromRawError> for DeviceError {
    fn from(error: FromRawError) -> Self {
        DeviceError::RawError(error)
    }
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let descr = match *self {
            DeviceError::ConstructionFailed => "DeviceError::ConstructionFailed",
            DeviceError::RawError(_) => "DeviceError::ConstructionFailed",
        };
        write!(f, "{}", descr)
    }
}

impl Error for DeviceError {
    fn description(&self) -> &str {
        match *self {
            DeviceError::ConstructionFailed => {
                "Could not create a default device. Please ensure that you are using at least OSX \
                 10.11 or iOS 8.0"
            }
            DeviceError::RawError(_) => "There was an error with the device pointer",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &DeviceError::ConstructionFailed => None,
            &DeviceError::RawError(ref raw_err) => Some(raw_err),
        }
    }
}
