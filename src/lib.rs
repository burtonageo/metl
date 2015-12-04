extern crate cocoa;
extern crate metal_sys;
#[macro_use]
extern crate objc;

mod device;
mod command_queue;
mod value_types;

pub mod sys {
    pub use metal_sys::*;
}

pub use device::{Device, DeviceError, DeviceRef};
pub use command_queue::{CommandQueue, CommandQueueError};
pub use value_types::{ClearColor, Origin, Region, ScissorRect, Size, Viewport};
