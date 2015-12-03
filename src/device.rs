use cocoa::base::{id, nil};
use cocoa::foundation::{NSUInteger, NSString};
use objc::runtime::{BOOL, NO, YES};
use objc::runtime::{Class, Object};
use sys::{MTLCreateSystemDefaultDevice, MTLDevice};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ffi::CStr;
use {Size};

pub trait ReadOnlyDevice {
    fn is_headless(&self) -> bool;
    fn is_low_power(&self) -> bool;
    fn max_threads_per_group(&self) -> bool;
    fn get_name(&self) -> Cow<str>;
    fn supports_texture_sample_count(&self, sample_count: usize) -> bool;
}

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

    pub unsafe fn get_raw(&self) -> id {self.0}

    pub unsafe fn into_raw(self) -> id {
        self.0
    }

    pub unsafe fn from_raw_unchecked(device_ptr: id) -> Self {
        Device(device_ptr)
    }

    pub fn from_raw(device_ptr: id) -> Result<Self, DeviceError> {
        #[link(name = "Foundation", kind = "framework")]
        extern {
            fn NSProtocolFromString(namestr: id) -> id;
        }

        let mtl_device_protocol_str = unsafe { NSString::alloc(nil).init_str("MTLDevice") };
        let mtl_device_protocol = unsafe { NSProtocolFromString(mtl_device_protocol_str) };

        let conforms_to_protocol: BOOL = unsafe { msg_send![device_ptr, conformsToProtocol:mtl_device_protocol] };

        if device_ptr == nil {
            Err(DeviceError::ConstructedFromNil)
        } else if conforms_to_protocol == NO {
            Err(DeviceError::ConstructedFromWrongPointerType)
        } else {
            Ok(Device(device_ptr))
        }
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
    ConstructionFailed,
    ConstructedFromNil,
    ConstructedFromWrongPointerType,
    OtherRuntimeError
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let descr = match *self {
            DeviceError::ConstructionFailed => "DeviceError::ConstructionFailed",
            DeviceError::ConstructedFromNil => "DeviceError::ConstructedFromNil",
            DeviceError::ConstructedFromWrongPointerType => "DeviceError::ConstructedFromWrongPointerType",
            DeviceError::OtherRuntimeError => "DeviceError::OtherRuntimeError"
        };
        write!(f, "{}", descr)
    }
}

impl Error for DeviceError {
    fn description(&self) -> &str {
        match *self {
            DeviceError::ConstructionFailed =>
                "Could not create a default device. Please ensure that you are using at least OSX 10.11 or iOS 8.0",
            DeviceError::ConstructedFromNil => "Attempted to create a device from a nil pointer",
            DeviceError::ConstructedFromWrongPointerType =>
                "Attempted to create a device from a pointer which does not implement MTLDevice",
            DeviceError::OtherRuntimeError => "An error occured in the Objective-C runtime"
        }
    }
}
