#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]

extern crate block;
extern crate cocoa;
extern crate core_foundation;
extern crate metal_sys;
#[macro_use]
extern crate objc;

pub mod extras;

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
mod depth_stencil_state;
mod device;
mod drawable;
mod error;
mod function;
mod library;
mod objc_bringup;
mod parallel_render_command_encoder;
mod render_command_encoder;
mod render_pass_attachment_descriptor;
mod render_pass_color_attachment_descriptor;
mod render_pass_depth_attachment_descriptor;
mod render_pass_descriptor;
mod render_pass_stencil_attachment_descriptor;
mod render_pipeline_state;
mod resource;
mod sampler_state;
mod stencil_descriptor;
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
pub use depth_stencil_state::{DepthStencilDescriptor, DepthStencilState};
pub use device::{BufferRef, Device, DeviceError, FeatureSet};
pub use drawable::Drawable;
pub use error::NSError;
pub use function::{Function, FunctionType};
pub use library::{Library, LibraryError};
pub use parallel_render_command_encoder::ParallelRenderCommandEncoder;
pub use raw::{AsRaw, FromRaw, FromRawError, IntoRaw, StrongPtr};
pub use render_command_encoder::{CullMode, DepthClipMode, IndexType, PrimitiveType,
                                 RenderCommandEncoder, TriangleFillModeFill, VisibilityResultMode,
                                 Winding};
pub use render_pass_attachment_descriptor::{LoadAction, RenderPassAttachmentDescriptor,
                                            StoreAction};
pub use render_pass_color_attachment_descriptor::{RenderPassColorAttachmentDescriptor,
                                                  RenderPassColorAttachmentDescriptorArray};
pub use render_pass_depth_attachment_descriptor::RenderPassDepthAttachmentDescriptor;
pub use render_pass_descriptor::RenderPassDescriptor;
pub use render_pass_stencil_attachment_descriptor::RenderPassStencilAttachmentDescriptor;
pub use render_pipeline_state::RenderPipelineState;
pub use resource::{CpuCacheMode, PurgeableState, Resource, ResourceOptions, StorageMode};
pub use sampler_state::{SamplerDescriptor, SamplerState};
pub use stencil_descriptor::StencilDescriptor;
pub use struct_member::StructMember;
pub use struct_type::StructType;
pub use texture::{Texture, TextureDescriptor, TextureType, TextureUsage};
pub use value_types::{ClearColor, Origin, Region, ScissorRect, Size, Viewport};
