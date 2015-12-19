use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use std::borrow::Cow;
use std::convert::{AsRef, From};
use std::ffi::CStr;
use std::io::Error as _IoError;
use sys::MTLLibrary;
use {FromRaw, FromRawError, Function};

pub struct Library(id);

impl Library {
    #[allow(unused_variables)]
    pub fn new_function_with_name<S: AsRef<str>>(&mut self, function_name: S) -> Option<Function> {
        unsafe {
            let func_name_nsstr = NSString::alloc(nil).init_str(function_name.as_ref());
            let function = self.0.newFunctionWithName(func_name_nsstr);
            FromRaw::from_raw(function).ok()
        }
    }

    pub fn function_names(&self) -> Vec<Cow<str>> {
        unimplemented!();
    }

    pub fn set_label<S: AsRef<str>>(&mut self, label: S) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label.as_ref())) }
    }

    pub fn get_label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.label().UTF8String()).to_string_lossy() }
    }
}

impl_from_into_raw!(Library, "MTLLibrary");

pub enum LibraryError {
    SourceError,
    FromRaw(FromRawError),
    IoError(_IoError)
}

impl From<FromRawError> for LibraryError {
    fn from(error: FromRawError) -> Self {
        LibraryError::FromRaw(error)
    }
}
