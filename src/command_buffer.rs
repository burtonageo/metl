use cocoa::base::{YES, id, nil};
use cocoa::foundation::NSString;
use error::NSError;
use std::borrow::Cow;
use std::convert::AsRef;
use std::error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
#[cfg(feature = "time2")]
use std::time::Instant;
use sys::{MTLCommandBuffer, MTLCommandBufferStatus};
use {AsRaw, BlitCommandEncoder, ComputeCommandEncoder, Drawable, FromRaw, FromRawError,
     ParallelRenderCommandEncoder, RenderCommandEncoder, RenderPassDescriptor};

pub struct CommandBuffer(id);

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

impl CommandBuffer {
    pub fn new_render_command_encoder(&mut self, descriptor: &RenderPassDescriptor)
                                      -> Result<RenderCommandEncoder, FromRawError> {
        let render_command_encoder = unsafe {
            self.0.renderCommandEncoderWithDescriptor(*descriptor.as_raw())
        };
        RenderCommandEncoder::from_raw(render_command_encoder)
    }

    pub fn new_blit_command_encoder(&mut self) -> Result<BlitCommandEncoder, FromRawError> {
        let blit_command_encoder = unsafe { self.0.blitCommandEncoder() };
        BlitCommandEncoder::from_raw(blit_command_encoder)
    }

    pub fn new_compute_command_encoder(&mut self) -> Result<ComputeCommandEncoder, FromRawError> {
        let compute_command_encoder = unsafe { self.0.computeCommandEncoder() };
        ComputeCommandEncoder::from_raw(compute_command_encoder)
    }

    pub fn new_parallel_render_command_encoder(
        &mut self, descriptor: &RenderPassDescriptor)
        -> Result<ParallelRenderCommandEncoder, FromRawError> {
        let par_render_command_encoder = unsafe {
            self.0.parallelRenderCommandEncoderWithDescriptor(*descriptor.as_raw())
        };
        ParallelRenderCommandEncoder::from_raw(par_render_command_encoder)
    }

    pub fn enqueue(&mut self) -> Result<(), CommandBufferError> {
        unsafe {
            self.0.enqueue();
        }
        self.get_error()
    }

    pub fn commit(&mut self) -> Result<(), CommandBufferError> {
        unsafe {
            self.0.commit();
        }
        self.get_error()
    }

    pub fn present_drawable(&mut self, drawable: &mut Drawable) -> Result<(), CommandBufferError> {
        unsafe {
            self.0.presentDrawable(*drawable.as_raw_mut());
        }
        self.get_error()
    }

    #[cfg(feature = "time2")]
    pub fn present_drawable_at_time(&mut self, drawable: &mut Drawable, time: Instant)
                                    -> Result<(), CommandBufferError> {
        unsafe {
            self.0.presentDrawable(*drawable.as_raw_mut(), time.elapsed().as_seconds());
        }
        self.get_error()
    }

    pub fn present_drawable_at_time_secs(&mut self, drawable: &mut Drawable, time: f64)
                                         -> Result<(), CommandBufferError> {
        unsafe {
            self.0.presentDrawable_atTime(*drawable.as_raw_mut(), time);
        }
        self.get_error()
    }

    pub fn wait_until_scheduled(&mut self) -> Result<(), CommandBufferError> {
        unsafe {
            self.0.waitUntilScheduled();
        }
        self.get_error()
    }

    pub fn wait_until_completed(&mut self) -> Result<(), CommandBufferError> {
        unsafe {
            self.0.waitUntilCompleted();
        }
        self.get_error()
    }

    pub fn status(&self) -> CommandBufferStatus {
        unsafe { self.0.status().into() }
    }

    pub fn has_retained_references(&self) -> bool {
        unsafe { self.0.retainedReferences() == YES }
    }

    pub fn set_label(&mut self, label: &AsRef<str>) {
        unsafe { MTLCommandBuffer::setLabel(self.0, NSString::alloc(nil).init_str(label.as_ref())) }
    }

    pub fn label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(MTLCommandBuffer::label(self.0).UTF8String()).to_string_lossy() }
    }

    fn get_error(&self) -> Result<(), CommandBufferError> {
        let error = unsafe { self.0.error() };
        if let Some(e) = NSError::new(error) { Err(CommandBufferError(e)) } else { Ok(()) }
    }
}

impl_from_into_raw!(CommandBuffer, of protocol "MTLCommandBuffer");

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum CommandBufferStatus: MTLCommandBufferStatus {
        CommandBufferStatusNotEnqueued => MTLCommandBufferStatusNotEnqueued,
        CommandBufferStatusEnqueued => MTLCommandBufferStatusEnqueued,
        CommandBufferStatusCommitted => MTLCommandBufferStatusCommitted,
        CommandBufferStatusScheduled => MTLCommandBufferStatusScheduled,
        CommandBufferStatusCompleted => MTLCommandBufferStatusCompleted,
        CommandBufferStatusError => MTLCommandBufferStatusError
    }
}

#[derive(Debug)]
pub struct CommandBufferError(NSError);

impl Display for CommandBufferError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0.domain())
    }
}

impl error::Error for CommandBufferError {
    fn description(&self) -> &str {
        self.0.localized_description()
    }
}
