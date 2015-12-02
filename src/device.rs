use cocoa::base::{id, nil};
use sys::{MTLCreateSystemDefaultDevice, MTLDevice};
use std::default::Default;
use std::ptr;

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
}
