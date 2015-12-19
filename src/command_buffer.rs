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

    pub fn set_label<S: AsRef<str>>(&mut self, label: S) {
        unsafe { MTLCommandBuffer::setLabel(self.0, NSString::alloc(nil).init_str(label.as_ref())) }
    }

    pub fn get_label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(MTLCommandBuffer::label(self.0).UTF8String()).to_string_lossy() }
    }
}

impl_from_into_raw!(CommandBuffer, "MTLCommandBuffer");

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum CommandBufferStatus {
    CommandBufferStatusNotEnqueued,
    CommandBufferStatusEnqueued,
    CommandBufferStatusCommitted,
    CommandBufferStatusScheduled,
    CommandBufferStatusCompleted,
    CommandBufferStatusError
}

impl From<MTLCommandBufferStatus> for CommandBufferStatus {
    fn from(mtl_status: MTLCommandBufferStatus) -> Self {
        match mtl_status {
            MTLCommandBufferStatus::MTLCommandBufferStatusNotEnqueued => {
                CommandBufferStatus::CommandBufferStatusNotEnqueued
            }
            MTLCommandBufferStatus::MTLCommandBufferStatusEnqueued => {
                CommandBufferStatus::CommandBufferStatusEnqueued
            }
            MTLCommandBufferStatus::MTLCommandBufferStatusCommitted => {
                CommandBufferStatus::CommandBufferStatusCommitted
            }
            MTLCommandBufferStatus::MTLCommandBufferStatusScheduled => {
                CommandBufferStatus::CommandBufferStatusScheduled
            }
            MTLCommandBufferStatus::MTLCommandBufferStatusCompleted => {
                CommandBufferStatus::CommandBufferStatusCompleted
            }
            MTLCommandBufferStatus::MTLCommandBufferStatusError => {
                CommandBufferStatus::CommandBufferStatusError
            }
        }
    }
}

impl Into<MTLCommandBufferStatus> for CommandBufferStatus {
    fn into(self) -> MTLCommandBufferStatus {
        match self {
            CommandBufferStatus::CommandBufferStatusNotEnqueued => {
                MTLCommandBufferStatus::MTLCommandBufferStatusNotEnqueued
            }
            CommandBufferStatus::CommandBufferStatusEnqueued => {
                MTLCommandBufferStatus::MTLCommandBufferStatusEnqueued
            }
            CommandBufferStatus::CommandBufferStatusCommitted => {
                MTLCommandBufferStatus::MTLCommandBufferStatusCommitted
            }
            CommandBufferStatus::CommandBufferStatusScheduled => {
                MTLCommandBufferStatus::MTLCommandBufferStatusScheduled
            }
            CommandBufferStatus::CommandBufferStatusCompleted => {
                MTLCommandBufferStatus::MTLCommandBufferStatusCompleted
            }
            CommandBufferStatus::CommandBufferStatusError => {
                MTLCommandBufferStatus::MTLCommandBufferStatusError
            }
        }
    }
}

pub struct CommandBufferError;
