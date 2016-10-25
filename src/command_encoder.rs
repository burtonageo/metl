use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use std::ffi::CStr;
use std::mem;
use sys::MTLCommandEncoder;
use Device;

#[derive(Debug)]
pub struct CommandEncoder(id);

impl CommandEncoder {
    pub fn end_encoding(&mut self) {
        unsafe { self.0.endEncoding(); }
    }

    pub fn insert_debug_signpost<S: AsRef<str>>(&mut self, signpost: S) {
        unsafe { self.0.insertDebugSignpost(NSString::alloc(nil).init_str(signpost.as_ref())) }
    }

    pub fn push_debug_group<S: AsRef<str>>(&mut self, debug_group: S) {
        unsafe { self.0.pushDebugGroup(NSString::alloc(nil).init_str(debug_group.as_ref())) }
    }

    pub fn pop_debug_group(&mut self) {
        unsafe { self.0.popDebugGroup(); }
    }

    pub fn device(&self) -> &Device {
        unsafe { mem::transmute(&self.0.device()) }
    }

    pub fn label(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.label().UTF8String()).to_str().unwrap_or(&"")
        }
    }
}

impl_from_into_raw!(CommandEncoder, of protocol "MTLCommandEncoder");
