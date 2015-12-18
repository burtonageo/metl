extern crate cocoa;
extern crate metal_sys;
#[macro_use]
extern crate objc;

mod command_buffer;
mod command_queue;
mod device;
mod drawable;
mod internal;
mod value_types;

pub mod sys {
    pub use metal_sys::*;
}

pub use command_buffer::CommandBuffer;
pub use command_queue::{CommandQueue, CommandQueueError};
pub use device::{Device, DeviceError};
pub use drawable::Drawable;
pub use value_types::{ClearColor, Origin, Region, ScissorRect, Size, Viewport};
