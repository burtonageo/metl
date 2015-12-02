use cocoa::base::id;

pub trait MTLCommandQueue {
    unsafe fn commandBuffer(self) -> id;
    unsafe fn commandBufferWithUnretainedReferences(self) -> id;
    unsafe fn insertDebugCaptureBoundary(self);

    unsafe fn getDevice(self) -> id;
    unsafe fn getLabel(self) -> id;
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
    
    unsafe fn getDevice(self) -> id {
        msg_send![self, getDevice]
    }

    unsafe fn getLabel(self) -> id {
        msg_send![self, getLabel]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }
}
