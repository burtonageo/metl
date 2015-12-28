use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc_bringup::NSError as _NSError;
use std::borrow::Cow;
use std::ffi::CStr;

pub struct NSError(id);

impl NSError {
    pub fn new(raw: id) -> Option<Self> {
        if raw == nil {
            None
        } else {
            Some(NSError(raw))
        }
    }

    pub fn code(&self) -> usize {
        unsafe { self.0.code() as usize }
    }

    pub fn domain(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.domain().UTF8String()).to_string_lossy() }
    }

    pub fn localized_description(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.localizedDescription().UTF8String()).to_string_lossy() }
    }
}
