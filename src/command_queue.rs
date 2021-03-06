use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::mem;
use std::marker::{Send, Sync};
use sys::MTLCommandQueue;
use {CommandBuffer, Device, FromRaw, FromRawError};

pub struct CommandQueue(id);

unsafe impl Send for CommandQueue {}
unsafe impl Sync for CommandQueue {}

impl CommandQueue {
    pub fn new_command_buffer(&mut self) -> CommandBuffer {
        let cmd_buf = unsafe { self.0.commandBuffer() };
        FromRaw::from_raw(cmd_buf).unwrap()
    }

    pub fn new_command_buffer_with_unretained_references(&mut self) -> CommandBuffer {
        let cmd_buf = unsafe { self.0.commandBufferWithUnretainedReferences() };
        FromRaw::from_raw(cmd_buf).unwrap()
    }

    pub fn insert_debug_capture_boundary(&mut self) {
        unsafe {
            self.0.insertDebugCaptureBoundary();
        }
    }

    pub fn set_label(&mut self, label: &str) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label)) }
    }

    pub fn label(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.label().UTF8String()).to_str().unwrap_or(&"") }
    }

    pub fn device(&self) -> &Device {
        let device = unsafe { self.0.device() };
        assert!(device != nil);
        unsafe { mem::transmute(device) }
    }
}

impl_from_into_raw!(CommandQueue, of protocol "MTLCommandQueue");

#[derive(Clone, Debug)]
pub enum CommandQueueError {
    CouldNotCreate
}

impl Display for CommandQueueError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let descr = match *self {
            CommandQueueError::CouldNotCreate => "DeviceError::CouldNotCreate",
        };
        write!(f, "{}", descr)
    }
}

impl Error for CommandQueueError {
    fn description(&self) -> &str {
        match *self {
            CommandQueueError::CouldNotCreate => "Could not create command queue.",
        }
    }
}

impl From<FromRawError> for CommandQueueError {
    fn from(_: FromRawError) -> Self {
        CommandQueueError::CouldNotCreate
    }
}
