use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use sys::{MTLCreateSystemDefaultDevice, MTLDevice};
use std::ffi::CString;
use std::ptr;
use std::str::FromStr;

pub struct Device(id);

impl Device {
    pub fn system_default_device() -> Result<Self, ()> {
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        if device != nil {
            Ok(Device(device))
        } else {
            Err(())
        }
    }

    pub fn is_headless(&self) -> bool {
        unsafe { self.0.headless() != 0 }
    }

    pub fn is_low_power(&self) -> bool {
        unsafe { self.0.lowPower() != 0 }
    }

    /// BEWARE: using this fn causes the process to exit abnormally.
    pub fn name(&self) -> String {
        let device_name_cstring = unsafe { CString::from_ptr(self.0.name().UTF8String()) };
        device_name_cstring.clone().into_string().unwrap_or(String::new())
    }
}
