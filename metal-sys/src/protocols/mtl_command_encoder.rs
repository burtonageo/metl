use cocoa::base::id;

pub trait MTLCommandEncoder {
    unsafe fn endEncoding(self);

    unsafe fn insertDebugSignpost(self, string: id);
    unsafe fn pushDebugGroup(self, string: id);
    unsafe fn popDebugGroup(self);

    unsafe fn device(self) -> id;
    unsafe fn label(self) -> id;
}

impl MTLCommandEncoder for id {
    unsafe fn endEncoding(self) {
        msg_send![self, endEncoding]
    }
    
    unsafe fn insertDebugSignpost(self, string: id) {
        msg_send![self, insertDebugSignpost:string]
    }

    unsafe fn pushDebugGroup(self, string: id) {
        msg_send![self, pushDebugGroup:string]
    }

    unsafe fn popDebugGroup(self) {
        msg_send![self, popDebugGroup]
    }
    
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }
}
