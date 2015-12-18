use cocoa::base::{YES, id, nil};
use cocoa::foundation::NSString;
use command_queue::_get_raw_command_queue;
use drawable::_drawable_get_id;
use std::borrow::Cow;
use std::convert::AsRef;
use std::ffi::CStr;
#[cfg(feature = "time2")]
use std::time::Instant;
use sys::{MTLCommandBuffer, MTLCommandBufferStatus, MTLCommandQueue};
use {CommandQueue, Drawable};

pub struct CommandBuffer(id);

pub struct CommandQueueRef;

impl CommandBuffer {
    pub fn new(command_queue: &mut CommandQueue) -> Self {
        let cmd_queue = unsafe { _get_raw_command_queue(command_queue) };
        let cmd_buf = unsafe { cmd_queue.commandBuffer() };
        debug_assert!(cmd_buf != nil);
        CommandBuffer(cmd_buf)
    }

    pub fn with_unretained_references(command_queue: &mut CommandQueue) -> Self {
        let cmd_queue = unsafe { _get_raw_command_queue(command_queue) };
        let cmd_buf_unretained = unsafe { cmd_queue.commandBufferWithUnretainedReferences() };
        debug_assert!(cmd_buf_unretained != nil);
        CommandBuffer(cmd_buf_unretained)
    }

    pub fn enqueue(&mut self) {
        unsafe { self.0.enqueue() }
    }

    pub fn commit(&mut self) {
        unsafe { self.0.commit() }
    }

    pub fn present_drawable(&mut self, drawable: Drawable) {
        unsafe { self.0.presentDrawable(_drawable_get_id(&drawable)) }
    }

    #[cfg(feature = "time2")]
    pub fn present_drawable_at_time(&mut self, drawable: &mut Drawable, time: Instant) {
        unsafe { self.0.presentDrawable(_drawable_get_id(&drawable), time.elapsed().as_seconds()) }
    }

    pub fn present_drawable_at_time_secs(&mut self, drawable: &mut Drawable, time: f64) {
        unsafe { self.0.presentDrawable_atTime(_drawable_get_id(&drawable), time) }
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

    pub fn get_error(&self) -> id {
        unsafe { self.0.error() }
    }

    pub fn has_retained_references(&self) -> bool {
        unsafe { self.0.retainedReferences() == YES }
    }

    pub fn get_command_queue(&self) -> CommandQueueRef {
        unimplemented!();
    }

    pub fn set_label<S: AsRef<str>>(&mut self, label: S) {
        unsafe { MTLCommandBuffer::setLabel(self.0, NSString::alloc(nil).init_str(label.as_ref())) }
    }

    pub fn get_label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(MTLCommandBuffer::label(self.0).UTF8String()).to_string_lossy() }
    }
}

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
            MTLCommandBufferStatus::MTLCommandBufferStatusNotEnqueued =>
                CommandBufferStatus::CommandBufferStatusNotEnqueued,
            MTLCommandBufferStatus::MTLCommandBufferStatusEnqueued =>
                CommandBufferStatus::CommandBufferStatusEnqueued,
            MTLCommandBufferStatus::MTLCommandBufferStatusCommitted =>
                CommandBufferStatus::CommandBufferStatusCommitted,
            MTLCommandBufferStatus::MTLCommandBufferStatusScheduled =>
                CommandBufferStatus::CommandBufferStatusScheduled,
            MTLCommandBufferStatus::MTLCommandBufferStatusCompleted =>
                CommandBufferStatus::CommandBufferStatusCompleted,
            MTLCommandBufferStatus::MTLCommandBufferStatusError =>
                CommandBufferStatus::CommandBufferStatusError,
        }
    }
}

impl Into<MTLCommandBufferStatus> for CommandBufferStatus {
    fn into(self) -> MTLCommandBufferStatus {
        match self {
            CommandBufferStatus::CommandBufferStatusNotEnqueued =>
                MTLCommandBufferStatus::MTLCommandBufferStatusNotEnqueued,
            CommandBufferStatus::CommandBufferStatusEnqueued =>
                MTLCommandBufferStatus::MTLCommandBufferStatusEnqueued,
            CommandBufferStatus::CommandBufferStatusCommitted =>
                MTLCommandBufferStatus::MTLCommandBufferStatusCommitted,
            CommandBufferStatus::CommandBufferStatusScheduled =>
                MTLCommandBufferStatus::MTLCommandBufferStatusScheduled,
            CommandBufferStatus::CommandBufferStatusCompleted =>
                MTLCommandBufferStatus::MTLCommandBufferStatusCompleted,
            CommandBufferStatus::CommandBufferStatusError =>
                MTLCommandBufferStatus::MTLCommandBufferStatusError,
        }
    }
}
