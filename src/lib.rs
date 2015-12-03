extern crate metal_sys;
extern crate cocoa;

mod device;
mod command_queue;
mod value_types;

pub mod sys {
    pub use metal_sys::*;
}

pub use device::{Device, DeviceError};
pub use command_queue::{CommandQueue, CommandQueueError};
pub use value_types::{ClearColor, Region, ScissorRect, Size, Viewport};
