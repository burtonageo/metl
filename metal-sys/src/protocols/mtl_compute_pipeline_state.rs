use cocoa::base::id;
use cocoa::foundation::NSUInteger;

pub trait MTLComputePipelineState {
    unsafe fn maxTotalThreadsPerThreadgroup(self) -> NSUInteger;
    unsafe fn threadExecutionWidth(self) -> NSUInteger;

    unsafe fn device(self) -> id;
}

impl MTLComputePipelineState for id {
    unsafe fn maxTotalThreadsPerThreadgroup(self) -> NSUInteger {
        msg_send![self, maxTotalThreadsPerThreadgroup]
    }

    unsafe fn threadExecutionWidth(self) -> NSUInteger {
        msg_send![self, threadExecutionWidth]
    }

    unsafe fn device(self) -> id {
        msg_send![self, device]
    }
}
