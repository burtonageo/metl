use cocoa::base::{id, nil};
use internal::conforms_to_protocol;
use {FromRaw, FromRawError, IntoRaw};

pub struct Function(id);

impl FromRaw for Function {
    fn from_raw(function_ptr: id) -> Result<Self, FromRawError> {
        if function_ptr == nil {
            Err(FromRawError::NilPointer)
        } else if unsafe { conforms_to_protocol(function_ptr, "MTLLibrary") } {
            Err(FromRawError::WrongPointerType)
        } else {
            Ok(Function(function_ptr))
        }
    }
}

impl IntoRaw for Function {
    fn into_raw(self) -> id {
        self.0
    }
}
