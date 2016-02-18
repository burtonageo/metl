use cocoa::base::id;
use cocoa::foundation::NSString;
use std::borrow::Cow;
use std::convert::From;
use std::ffi::CStr;
use sys::{MTLFunction, MTLFunctionType};

pub struct Function(id);

impl Function {
    pub fn name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_string_lossy() }
    }

    pub fn function_type(&self) -> FunctionType {
        unsafe { From::from(self.0.functionType()) }
    }

    // TODO(George): Model this correctly
    pub fn vertex_attributes(&self) -> ! {
        unimplemented!();
    }
}

impl_from_into_raw!(Function, of protocol "MTLFunction");

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    enum FunctionType: MTLFunctionType {
        Vertex => MTLFunctionTypeVertex,
        Fragment => MTLFunctionTypeFragment,
        Kernel => MTLFunctionTypeKernel
    }
}
