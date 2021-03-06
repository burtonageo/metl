use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;

pub trait MTLRenderPassDescriptor {
    /// Creates a default render pass descriptor.
    ///
    /// # Description
    ///
    /// Set the desired color attachments with the `setObject:atIndexedSubscript:`
    /// method of the `colorAttachments` property. Set the desired depth and stencil
    /// attachments with the `depthAttachment` and `stencilAttachment` properties,
    /// respectively.
    unsafe fn renderPassDescriptor(_: Self) -> id {
        msg_send![class("MTLRenderPassDescriptor"), renderPassDescriptor]
    }

    /// An array of state information for attachments that store color data.
    unsafe fn colorAttachments(self) -> id;
    unsafe fn setColorAttachments(self, colorAttachments: id);

    /// State information for an attachment that stores depth data.
    unsafe fn depthAttachment(self) -> id;
    unsafe fn setDepthAttachment(self, depthAttachment: id);

    /// State information for an attachment that stores stencil data.
    unsafe fn stencilAttachment(self) -> id;
    unsafe fn setStencilAttachment(self, stencilAttachment: id);

    /// The destination for the GPU to write visibility information when samples
    /// pass the depth and stencil tests.
    unsafe fn visibilityResultBuffer(self) -> id;
    unsafe fn setVisibilityResultBuffer(self, visibilityResultBuffer: id);

    #[cfg(target_os = "macos")]
    unsafe fn renderTargetArrayLength(self) -> NSUInteger;

    #[cfg(target_os = "macos")]
    unsafe fn setRenderTargetArrayLength(self, renderTargetArrayLength: NSUInteger);

    unsafe fn copy(self) -> id;
}

impl MTLRenderPassDescriptor for id {
    unsafe fn colorAttachments(self) -> id {
        msg_send![self, colorAttachments]
    }

    unsafe fn setColorAttachments(self, colorAttachments: id) {
        msg_send![self, setColorAttachments:colorAttachments]
    }

    unsafe fn depthAttachment(self) -> id {
        msg_send![self, depthAttachment]
    }

    unsafe fn setDepthAttachment(self, depthAttachment: id) {
        msg_send![self, setDepthAttachment:depthAttachment]
    }

    unsafe fn stencilAttachment(self) -> id {
        msg_send![self, stencilAttachment]
    }

    unsafe fn setStencilAttachment(self, stencilAttachment: id) {
        msg_send![self, setStencilAttachment:stencilAttachment]
    }

    unsafe fn visibilityResultBuffer(self) -> id {
        msg_send![self, visibilityResultBuffer]
    }

    unsafe fn setVisibilityResultBuffer(self, visibilityResultBuffer: id) {
        msg_send![self, setVisibilityResultBuffer:visibilityResultBuffer]
    }

    #[cfg(target_os = "macos")]
    unsafe fn renderTargetArrayLength(self) -> NSUInteger {
        msg_send![self, renderTargetArrayLength]
    }

    #[cfg(target_os = "macos")]
    unsafe fn setRenderTargetArrayLength(self, renderTargetArrayLength: NSUInteger) {
        msg_send![self, setRenderTargetArrayLength:renderTargetArrayLength]
    }

    unsafe fn copy(self) -> id {
        msg_send![self, copy]
    }
}
