use cocoa::base::id;
use cocoa::foundation::NSString;
use std::borrow::Cow;
use std::convert::{From, Into};
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

impl_from_into_raw!(Function, "MTLFunction");

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum FunctionType {
    Vertex,
    Fragment,
    Kernel
}

impl From<MTLFunctionType> for FunctionType {
    fn from(function_type: MTLFunctionType) -> Self {
        match function_type {
            MTLFunctionType::MTLFunctionTypeVertex => FunctionType::Vertex,
            MTLFunctionType::MTLFunctionTypeFragment => FunctionType::Fragment,
            MTLFunctionType::MTLFunctionTypeKernel => FunctionType::Kernel,
        }
    }
}

impl Into<MTLFunctionType> for FunctionType {
    fn into(self) -> MTLFunctionType {
        match self {
            FunctionType::Vertex => MTLFunctionType::MTLFunctionTypeVertex,
            FunctionType::Fragment => MTLFunctionType::MTLFunctionTypeFragment,
            FunctionType::Kernel => MTLFunctionType::MTLFunctionTypeKernel,
        }
    }
}
