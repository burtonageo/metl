use cocoa::base::{YES, id, nil};
use cocoa::foundation::NSString;
use std::borrow::Cow;
use std::convert::AsRef;
use std::ffi::CStr;
#[cfg(feature = "time2")]
use std::time::Instant;
use sys::{MTLCommandBuffer, MTLCommandBufferStatus};
use {AsRaw, Drawable};

pub struct CommandBuffer(id);

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

impl CommandBuffer {
    pub fn enqueue(&mut self) {
        unsafe { self.0.enqueue() }
    }

    pub fn commit(&mut self) -> Result<(), CommandBufferError> {
        unsafe {
            self.0.commit();
            let error = self.0.error();
            if error != nil { Err(CommandBufferError) } else { Ok(()) }
        }
    }

    pub fn present_drawable(&mut self, drawable: &mut Drawable) {
        unsafe { self.0.presentDrawable(*drawable.as_raw_mut()) }
    }

    #[cfg(feature = "time2")]
    pub fn present_drawable_at_time(&mut self, drawable: &mut Drawable, time: Instant) {
        unsafe { self.0.presentDrawable(*drawable.as_raw_mut(), time.elapsed().as_seconds()) }
    }

    pub fn present_drawable_at_time_secs(&mut self, drawable: &mut Drawable, time: f64) {
        unsafe { self.0.presentDrawable_atTime(*drawable.as_raw_mut(), time) }
    }

    pub fn wait_until_scheduled(&mut self) {
        unsafe { self.0.waitUntilScheduled() }
    }

    pub fn wait_until_completed(&mut self) {
        unsafe { self.0.waitUntilCompleted() }
    }

    pub fn get_status(&self) -> CommandBufferStatus {
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
}

impl_from_into_raw!(CommandBuffer, of protocol "MTLCommandBuffer");

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum CommandBufferStatus : MTLCommandBufferStatus {
        CommandBufferStatusNotEnqueued => MTLCommandBufferStatusNotEnqueued,
        CommandBufferStatusEnqueued => MTLCommandBufferStatusEnqueued,
        CommandBufferStatusCommitted => MTLCommandBufferStatusCommitted,
        CommandBufferStatusScheduled => MTLCommandBufferStatusScheduled,
        CommandBufferStatusCompleted => MTLCommandBufferStatusCompleted,
        CommandBufferStatusError => MTLCommandBufferStatusError
    }
}

pub struct CommandBufferError;
