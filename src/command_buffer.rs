use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use command_queue::_get_raw_command_queue;
use device::_make_device_ref;
use std::borrow::Cow;
use std::convert::AsRef;
use std::ffi::CStr;
use sys::{MTLCommandBuffer, MTLCommandQueue, MTLCommandBufferStatus};
use {CommandQueue, DeviceRef};

pub type CommandBufferStatus = MTLCommandBufferStatus;

pub struct CommandBuffer(id);

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

    pub fn set_label<S: AsRef<str>>(&mut self, label: S) {
        unsafe { MTLCommandBuffer::setLabel(self.0, NSString::alloc(nil).init_str(label.as_ref())) }
    }
    
    pub fn get_label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(MTLCommandBuffer::label(self.0).UTF8String()).to_string_lossy() }
    }
    
    pub fn get_device<'a>(&'a self) -> DeviceRef<'a> {
        let device = unsafe { MTLCommandBuffer::device(self.0) };
        debug_assert!(device != nil);
        unsafe { _make_device_ref(device) }
    }
}
