use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use std::borrow::Cow;
use std::convert::From;
use std::ffi::CStr;
use std::mem;
use sys::{MTLFunction, MTLFunctionType};
use Device;

pub struct Function(id);

impl Function {
    pub fn name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_string_lossy() }
    }

    pub fn device(&self) -> &Device {
        let device = self.0.device();
        assert!(device != nil);
        unsafe { mem::transmute(device) }
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
    pub enum FunctionType: MTLFunctionType {
        Vertex => MTLFunctionTypeVertex,
        Fragment => MTLFunctionTypeFragment,
        Kernel => MTLFunctionTypeKernel
    }
}
