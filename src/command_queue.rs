use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use internal::conforms_to_protocol;
use std::borrow::Cow;
use std::convert::AsRef;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use sys::MTLCommandQueue;
use {CommandBuffer, FromRaw, FromRawError, IntoRaw};

pub struct CommandQueue(id);

impl CommandQueue {
    pub fn command_buffer(&mut self) -> CommandBuffer {
        let cmd_buf = unsafe { self.0.commandBuffer() };
        FromRaw::from_raw(cmd_buf).unwrap()
    }

    pub fn command_buffer_with_unretained_references(&mut self) -> CommandBuffer {
        let cmd_buf = unsafe { self.0.commandBufferWithUnretainedReferences() };
        FromRaw::from_raw(cmd_buf).unwrap()
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

impl FromRaw for CommandQueue {
    fn from_raw(cmd_queue_ptr: id) -> Result<Self, FromRawError> {
        if cmd_queue_ptr == nil {
            Err(FromRawError::NilPointer)
        } else if unsafe { conforms_to_protocol(cmd_queue_ptr, "MTLCommandQueue") } {
            Err(FromRawError::WrongPointerType)
        } else {
            Ok(CommandQueue(cmd_queue_ptr))
        }
    }
}

impl IntoRaw for CommandQueue {
    fn into_raw(self) -> id {
        self.0
    }
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

impl From<FromRawError> for CommandQueueError {
    fn from(_: FromRawError) -> Self {
        CommandQueueError::CouldNotCreate
    }
}
