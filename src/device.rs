use cocoa::base::{id, nil};
use cocoa::foundation::{NSUInteger, NSString};
use sys::{MTLCreateSystemDefaultDevice, MTLDevice};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ffi::CStr;
use {Size};

pub struct Device(id);

impl Device {
    pub fn system_default_device() -> Result<Self, DeviceError> {
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        if device != nil {
            Ok(Device(device))
        } else {
            Err(DeviceError::ConstructionFailed)
        }
    }

    pub unsafe fn get_raw(&self) -> id {
        self.0
    }

    pub fn is_headless(&self) -> bool {
        unsafe { self.0.headless() != 0 }
    }

    pub fn is_low_power(&self) -> bool {
        unsafe { self.0.lowPower() != 0 }
    }

    pub fn max_threads_per_group(&self) -> Size {
        unsafe { self.0.maxThreadsPerGroup().into() }
    }

    pub fn get_name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_string_lossy() }
    }

    pub fn supports_texture_sample_count(&self, sample_count: usize) -> bool {
        unsafe { self.0.supportsTextureSampleCount(sample_count as NSUInteger) != 0 }
    }
}

/// Internal utility function to create a Device without exposing internals.
/// Not exported publicly from this crate.
#[doc(hidden)]
pub unsafe fn _make_device(device: id) -> Device {
    Device(device)
}

#[derive(Clone, Debug)]
pub enum DeviceError {
    ConstructionFailed
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let descr = match *self {
            DeviceError::ConstructionFailed => "DeviceError::ConstructionFailed"
        };
        write!(f, "{}", descr)
    }
}

impl Error for DeviceError {
    fn description(&self) -> &str {
        match *self {
            DeviceError::ConstructionFailed =>
                "Could not create a default device. Please ensure that you are using at least OSX 10.11 or iOS 8.0"
        }
    }
}
