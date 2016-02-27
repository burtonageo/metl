#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]

extern crate cocoa;
extern crate core_foundation;
extern crate metal_sys;
#[macro_use]
extern crate objc;

#[macro_use]
mod raw;

#[macro_use]
mod internal;

mod argument;
mod array_type;
mod blit_command_encoder;
mod buffer;
mod command_buffer;
mod command_queue;
mod compile_options;
mod compute_command_encoder;
mod constants;
mod device;
mod drawable;
mod error;
mod function;
mod library;
mod objc_bringup;
mod resource;
mod struct_member;
mod struct_type;
mod texture;
mod value_types;

pub mod sys {
    pub use metal_sys::*;
}

pub use argument::{Argument, ArgumentAccess, ArgumentType, DataType};
pub use array_type::ArrayType;
pub use blit_command_encoder::BlitCommandEncoder;
pub use buffer::Buffer;
pub use command_buffer::{CommandBuffer, CommandBufferError};
pub use command_queue::{CommandQueue, CommandQueueError};
pub use compile_options::{CompileOptions, LanguageVersion, PreprocessorMacroValue,
                          SpecificLanguageVersion};
pub use compute_command_encoder::ComputeCommandEncoder;
pub use constants::{CompareFunction, PipelineOption, PixelFormat};
pub use device::{Device, DeviceError, FeatureSet};
pub use drawable::Drawable;
pub use error::NSError;
pub use function::{Function, FunctionType};
pub use library::{Library, LibraryError, LibraryErrorType};
pub use raw::{AsRaw, FromRaw, FromRawError, IntoRaw};
pub use resource::Resource;
pub use struct_member::StructMember;
pub use struct_type::StructType;
pub use texture::{Texture, TextureType, TextureUsage};
pub use value_types::{ClearColor, Origin, Region, ScissorRect, Size, Viewport};
