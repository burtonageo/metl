use cocoa::base::id;
use objc::runtime::BOOL;
use MTLCompareFunction;

pub trait MTLDepthStencilStateDescriptor {
    unsafe fn depthCompareFunction(self) -> MTLCompareFunction;
    unsafe fn setDepthCompareFunction(self, depthCompareFunction: MTLCompareFunction);

    unsafe fn depthWriteEnabled(self) -> BOOL;
    unsafe fn setDepthWriteEnabled(self, depthWriteEnabled: BOOL);

    unsafe fn backFaceStencil(self) -> id;
    unsafe fn setBackFaceStencil(self, backFaceStencil: id);

    unsafe fn frontFaceStencil(self) -> id;
    unsafe fn setFrontFaceStencil(self, frontFaceStencil: id);

    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);
}

impl MTLDepthStencilStateDescriptor for id {
    unsafe fn depthCompareFunction(self) -> MTLCompareFunction {
        msg_send![self, depthCompareFunction]
    }

    unsafe fn setDepthCompareFunction(self, depthCompareFunction: MTLCompareFunction) {
        msg_send![self, setDepthCompareFunction:depthCompareFunction]
    }

    unsafe fn depthWriteEnabled(self) -> BOOL {
        msg_send![self, depthWriteEnabled]
    }

    unsafe fn setDepthWriteEnabled(self, depthWriteEnabled: BOOL) {
        msg_send![self, setDepthWriteEnabled:depthWriteEnabled]
    }

    unsafe fn backFaceStencil(self) -> id {
        msg_send![self, backFaceStencil]
    }

    unsafe fn setBackFaceStencil(self, backFaceStencil: id) {
        msg_send![self, setBackFaceStencil:backFaceStencil]
    }

    unsafe fn frontFaceStencil(self) -> id {
        msg_send![self, frontFaceStencil]
    }

    unsafe fn setFrontFaceStencil(self, frontFaceStencil: id) {
        msg_send![self, setFrontFaceStencil:frontFaceStencil]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }
}
