use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc_bringup::NSError as _NSError;
use std::ffi::CStr;

#[derive(Debug)]
pub struct NSError(id);

impl NSError {
    pub fn new(raw: id) -> Option<Self> {
        if raw == nil { None } else { Some(NSError(raw)) }
    }

    pub fn code(&self) -> usize {
        unsafe { self.0.code() as usize }
    }

    pub fn domain(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.domain().UTF8String()).to_str().unwrap_or(&"") }
    }

    pub fn localized_description(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.localizedDescription().UTF8String()).to_str().unwrap_or(&"") }
    }
}
