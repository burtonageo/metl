use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use command_buffer::_make_command_buffer;
use std::borrow::Cow;
use std::convert::AsRef;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use sys::MTLCommandQueue;
use CommandBuffer;

pub struct CommandQueue(id);

impl CommandQueue {
    pub fn command_buffer(&mut self) -> CommandBuffer {
        let cmd_buf = unsafe { self.0.commandBuffer() };
        debug_assert!(cmd_buf != nil);
        _make_command_buffer(cmd_buf)
    }

    pub fn command_buffer_with_unretained_references(&mut self) -> CommandBuffer {
        let cmd_buf = unsafe { self.0.commandBufferWithUnretainedReferences() };
        debug_assert!(cmd_buf != nil);
        _make_command_buffer(cmd_buf)
    }

    pub fn insert_debug_capture_boundary(&mut self) {
        unsafe {
            self.0.insertDebugCaptureBoundary();
        }
    }

    pub fn set_label<S: AsRef<str>>(&mut self, label: S) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label.as_ref())) }
    }

    pub fn get_label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.label().UTF8String()).to_string_lossy() }
    }
}

/// Internal utility function to create a CommandQueue without exposing internals.
/// Not exported publicly from this crate.
#[doc(hidden)]
pub fn _make_command_queue(command_queue: id) -> CommandQueue {
    CommandQueue(command_queue)
}

/// Internal utility function to get a CommandQueue's id without consuming it.
/// Not exported publicly from this crate.
#[doc(hidden)]
pub unsafe fn _get_raw_command_queue(command_queue: &CommandQueue) -> id {
    command_queue.0
}

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
