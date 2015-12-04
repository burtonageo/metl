use cocoa::base::{id, nil};
use cocoa::foundation::{NSUInteger, NSString};
use objc::runtime::{BOOL, NO, YES};
use sys::{MTLCreateSystemDefaultDevice, MTLCopyAllDevices, MTLDevice};
use std::borrow::Cow;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use {Size};

pub trait ReadOnlyDevice {
    fn is_depth24_stencil8_pixel_format_supported(&self) -> bool;
    fn is_headless(&self) -> bool;
    fn is_low_power(&self) -> bool;
    fn max_threads_per_group(&self) -> Size;
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

    #[allow(unused_mut, unused_variables)]
    pub fn enumerate_all_system_devices() -> Vec<Result<Self, DeviceError>> {
        let all_devices = unsafe { MTLCopyAllDevices() };
        let mut devices_vec = vec![];



        devices_vec
    }

    /// Get the underlying pointer to the device. Releases ownership.
    pub unsafe fn into_raw(self) -> id {
        self.0
    }

    /// Create a `Device` from a raw pointer. Does no error checking, so it
    /// will cause errors if the device is nil, or does not conform to the
    /// MTLDevice protocol.
    pub unsafe fn from_raw_unchecked(device_ptr: id) -> Self {
        Device(device_ptr)
    }

    /// Create a `Device` from a raw pointer. If the pointer is not nil and
    /// conforms to the `MTLDevice` protocol, then a `Device` will be created,
    /// otherwise returns a `DeviceError`.
    pub fn from_raw(device_ptr: id) -> Result<Self, DeviceError> {
        #[link(name = "Foundation", kind = "framework")]
        extern {
            fn NSProtocolFromString(namestr: id) -> id;
        }

        let conforms_to_protocol: BOOL = unsafe {
            let mtl_device_protocol_str = NSString::alloc(nil).init_str("MTLDevice");
            let mtl_device_protocol = NSProtocolFromString(mtl_device_protocol_str);
            msg_send![device_ptr, conformsToProtocol:mtl_device_protocol]
        };

        if device_ptr == nil {
            Err(DeviceError::ConstructedFromNil)
        } else if conforms_to_protocol == NO {
            Err(DeviceError::ConstructedFromWrongPointerType)
        } else {
            Ok(Device(device_ptr))
        }
    }
}

impl ReadOnlyDevice for Device {
    fn is_depth24_stencil8_pixel_format_supported(&self) -> bool {
        unsafe { self.0.depth24Stencil8PixelFormatSupported() == YES }
    }

    fn is_headless(&self) -> bool {
        unsafe { self.0.headless() == YES }
    }

    fn is_low_power(&self) -> bool {
        unsafe { self.0.lowPower() == YES }
    }

    fn max_threads_per_group(&self) -> Size {
        unsafe { self.0.maxThreadsPerGroup().into() }
    }

    fn get_name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_string_lossy() }
    }

    fn supports_texture_sample_count(&self, sample_count: usize) -> bool {
        unsafe { self.0.supportsTextureSampleCount(sample_count as NSUInteger) == YES }
    }
}

/// Internal utility function to get a Device's id without consuming it.
/// Not exported publicly from this crate.
#[doc(hidden)]
pub unsafe fn _device_get_raw(device: &Device) -> id {
    device.0
}

// TODO(George): Should this have a lifetime associated with it?
pub struct DeviceRef(id);

impl DeviceRef {
    pub fn is_reference_to(&self, device: &Device) -> bool {
        self.0 == device.0
    }
}

impl ReadOnlyDevice for DeviceRef {
    fn is_depth24_stencil8_pixel_format_supported(&self) -> bool {
        unsafe { self.0.depth24Stencil8PixelFormatSupported() == YES }
    }

    fn is_headless(&self) -> bool {
        unsafe { self.0.headless() == YES }
    }

    fn is_low_power(&self) -> bool {
        unsafe { self.0.lowPower() == YES }
    }

    fn max_threads_per_group(&self) -> Size {
        unsafe { self.0.maxThreadsPerGroup().into() }
    }

    fn get_name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_string_lossy() }
    }

    fn supports_texture_sample_count(&self, sample_count: usize) -> bool {
        unsafe { self.0.supportsTextureSampleCount(sample_count as NSUInteger) == YES }
    }
}

/// Internal utility function to create a DeviceRef.
/// Not exported publicly from this crate.
#[doc(hidden)]
pub unsafe fn _make_device_ref(device: id) -> DeviceRef {
    DeviceRef(device)
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
