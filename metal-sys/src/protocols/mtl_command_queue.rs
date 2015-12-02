use cocoa::base::id;

pub trait MTLCommandQueue {
    unsafe fn commandBuffer(self) -> id;
    unsafe fn commandBufferWithUnretainedReferences(self) -> id;
    unsafe fn insertDebugCaptureBoundary(self);

    unsafe fn device(self) -> id;
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, id);
}

impl MTLCommandQueue for id {
    unsafe fn commandBuffer(self) -> id {
        msg_send![self, commandBuffer]
    }

    unsafe fn commandBufferWithUnretainedReferences(self) -> id {
        msg_send![self, commandBufferWithUnretainedReferences]
    }

    unsafe fn insertDebugCaptureBoundary(self) {
        msg_send![self, insertDebugCaptureBoundary]
    }
    
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }
}
