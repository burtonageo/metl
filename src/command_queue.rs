use cocoa::base::{id, nil};
use cocoa::foundation::{NSString, NSUInteger};
use device::_make_device;
use std::borrow::Cow;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use sys::{MTLCommandQueue, MTLDevice};
use {Device};

pub struct CommandQueue(id);

impl CommandQueue {
    pub fn new(device: &Device) -> Result<Self, CommandQueueError> {
        let command_queue = unsafe { device.get_raw().newCommandQueue() };
        if command_queue != nil {
            Ok(CommandQueue(command_queue))
        } else {
            Err(CommandQueueError::CouldNotCreate)
        }
    }

    pub fn with_max_command_buffer_count(device: &Device,
                                         max_command_buffer_count: usize) -> Result<Self, CommandQueueError> {
        let command_queue = unsafe {
            device.get_raw().newCommandQueueWithMaxCommandBufferCount(max_command_buffer_count as NSUInteger)
        };
        if command_queue != nil {
            Ok(CommandQueue(command_queue))
        } else {
            Err(CommandQueueError::CouldNotCreate)
        }
    }

    pub fn insert_debug_capture_boundary(&self) {
        unsafe { self.0.insertDebugCaptureBoundary(); }
    }

    pub fn set_label(&self, label: &str) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label)) }
    }

    pub fn get_label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.label().UTF8String()).to_string_lossy() }
    }

    pub fn get_device(&self) -> Device {
        unsafe { _make_device(self.0.device()) }
    }
}

/// Internal utility function to create a CommandQueue without exposing internals.
/// Not exported publicly from this crate.
#[doc(hidden)]
pub fn _make_command_queue(command_queue: id) -> CommandQueue {
    CommandQueue(command_queue)
}

#[derive(Clone, Debug)]
pub enum CommandQueueError {
    CouldNotCreate
}

impl Display for CommandQueueError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let descr = match *self {
            CommandQueueError::CouldNotCreate => "DeviceError::CouldNotCreate"
        };
        write!(f, "{}", descr)
    }
}

impl Error for CommandQueueError {
    fn description(&self) -> &str {
        match *self {
            CommandQueueError::CouldNotCreate => "Could not create command queue."
        }
    }
}
