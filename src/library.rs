use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use error::NSError;
use objc_bringup::NSArray;
use std::convert::From;
use std::error::Error;
use std::ffi::CStr;
use std::fmt;
use std::io::Error as IoError;
use std::mem;
use std::sync::Arc;
use sys::MTLLibrary;
use {Device, FromRaw, FromRawError, Function};

pub struct Library(id);

impl Library {
    pub fn new_function_with_name(&mut self, function_name: &str) -> Option<Function> {
        unsafe {
            let func_name_nsstr = NSString::alloc(nil).init_str(function_name);
            let function = self.0.newFunctionWithName(func_name_nsstr);
            FromRaw::from_raw(function).ok()
        }
    }

    pub fn function_names(&self) -> Vec<&str> {
        let names_array = unsafe { self.0.functionNames() };
        let names_len = unsafe { names_array.count() };
        let mut names_vec = vec![];
        for i in 0..names_len {
            let name = unsafe {
                CStr::from_ptr(names_array.objectAtIndex(i).UTF8String()).to_str().unwrap_or(&"")
            };
            names_vec.push(name);
        }
        names_vec
    }

    pub fn device(&self) -> &Device {
        let device = unsafe { self.0.device() };
        assert!(device != nil);
        unsafe { mem::transmute(device) }
    }

    pub fn set_label(&mut self, label: &str) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label)) }
    }

    pub fn label(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.label().UTF8String()).to_str().unwrap_or(&"") }
    }
}

impl_from_into_raw!(Library, of protocol "MTLLibrary");

#[derive(Debug)]
pub enum LibraryError {
    SourceError(Option<Arc<NSError>>),
    FromRaw(FromRawError),
    Io(IoError)
}

impl fmt::Display for LibraryError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description())
    }
}

impl Error for LibraryError {
    fn description(&self) -> &str {
        match *self {
            LibraryError::SourceError(Some(ref e)) => e.domain(),
            LibraryError::SourceError(None) => "Could not parse source code.",
            LibraryError::FromRaw(_) => "Error converting library from pointer",
            LibraryError::Io(_) => "Io error while loading library",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            LibraryError::FromRaw(ref e) => {
                let e: &Error = e;
                Some(e)
            }
            LibraryError::Io(ref e) => {
                let e: &Error = e;
                Some(e)
            }
            _ => None,
        }
    }
}

impl From<NSError> for LibraryError {
    fn from(error: NSError) -> Self {
        LibraryError::SourceError(Some(Arc::new(error)))
    }
}

impl From<Option<NSError>> for LibraryError {
    fn from(error: Option<NSError>) -> Self {
        if let Some(error) = error {
            LibraryError::SourceError(Some(Arc::new(error)))
        } else {
            LibraryError::SourceError(None)
        }
    }
}

impl From<FromRawError> for LibraryError {
    fn from(error: FromRawError) -> Self {
        LibraryError::FromRaw(error)
    }
}

impl From<IoError> for LibraryError {
    fn from(error: IoError) -> Self {
        LibraryError::Io(error)
    }
}
