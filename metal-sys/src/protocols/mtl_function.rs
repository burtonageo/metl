use cocoa::base::id;

pub trait MTLFunction {
    unsafe fn name(self) -> id;
    unsafe fn functionType(self) -> MTLFunctionType;
    unsafe fn device(self) -> id;
    unsafe fn vertexAttributes(self) -> id;
}

impl MTLFunction for id {
    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn functionType(self) -> MTLFunctionType {
        msg_send![self, functionType]
    }

    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn vertexAttributes(self) -> id {
        msg_send![self, vertexAttributes]
    }
}

#[repr(C, usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLFunctionType {
    MTLFunctionTypeVertex = 1,
    MTLFunctionTypeFragment = 2,
    MTLFunctionTypeKernel = 3
}
