use cocoa::base::id;

pub trait MTLDepthStencilState {
    unsafe fn device(self) -> id;
    unsafe fn label(self) -> id;
}

impl MTLDepthStencilState for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }
}
