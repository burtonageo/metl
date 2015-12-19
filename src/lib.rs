extern crate cocoa;
extern crate metal_sys;
#[macro_use]
extern crate objc;

#[macro_use]
mod raw;

mod command_buffer;
mod command_queue;
mod compile_options;
mod device;
mod drawable;
mod function;
mod internal;
mod library;
mod value_types;

pub mod sys {
    pub use metal_sys::*;
}

pub use command_buffer::CommandBuffer;
pub use command_queue::{CommandQueue, CommandQueueError};
pub use compile_options::{CompileOptions, PreprocessorMacro};
pub use device::{Device, DeviceError};
pub use drawable::Drawable;
pub use function::Function;
pub use library::{Library, LibraryError};
pub use raw::{AsRaw, FromRaw, FromRawError, IntoRaw};
pub use value_types::{ClearColor, Origin, Region, ScissorRect, Size, Viewport};
