use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use internal::conforms_to_protocol;
use std::borrow::Cow;
use std::convert::AsRef;
use std::ffi::CStr;
use sys::MTLLibrary;
use {FromRaw, FromRawError, Function, IntoRaw};

pub struct Library(id);

impl Library {
    #[allow(unused_variables)]
    pub fn new_function_with_name<S: AsRef<str>>(&mut self, function_name: S) -> Function {
        unsafe {
            let func_name_nsstr = NSString::alloc(nil).init_str(function_name.as_ref());
            let function = self.0.newFunctionWithName(func_name_nsstr);
            FromRaw::from_raw(function).unwrap()
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

impl FromRaw for Library {
    fn from_raw(library_ptr: id) -> Result<Self, FromRawError> {
        if library_ptr == nil {
            Err(FromRawError::NilPointer)
        } else if unsafe { conforms_to_protocol(library_ptr, "MTLLibrary") } {
            Err(FromRawError::WrongPointerType)
        } else {
            Ok(Library(library_ptr))
        }
    }
}

impl IntoRaw for Library {
    fn into_raw(self) -> id {
        self.0
    }
}
