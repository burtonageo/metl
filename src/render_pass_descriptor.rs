use cocoa::base::{id, nil};
use cocoa::foundation::NSUInteger;
use sys::MTLRenderPassDescriptor;
use {FromRaw, IntoRaw, RenderPassColorAttachmentDescriptor,
     RenderPassColorAttachmentDescriptorArray, RenderPassDepthAttachmentDescriptor,
     RenderPassStencilAttachmentDescriptor};

pub struct RenderPassDescriptor(id);

impl RenderPassDescriptor {
    pub fn new() -> Self {
        unsafe { FromRaw::from_raw(MTLRenderPassDescriptor::renderPassDescriptor(nil)).unwrap() }
    }

    pub fn color_attachments(&self) -> RenderPassColorAttachmentDescriptorArray {
        unsafe { FromRaw::from_raw(self.0.colorAttachments()).unwrap() }
    }

    pub fn set_color_attachments(&mut self, attachments: RenderPassColorAttachmentDescriptorArray) {
        unsafe { self.0.setColorAttachments(attachments.into_raw()) }
    }

    pub fn depth_attachment(&self) -> RenderPassDepthAttachmentDescriptor {
        unsafe { FromRaw::from_raw(self.0.depthAttachment()).unwrap() }
    }

    pub fn set_depth_attachment(&mut self, attachment: RenderPassDepthAttachmentDescriptor) {
        unsafe { self.0.setDepthAttachment(attachment.into_raw()) }
    }

    pub fn stencil_attachment(&self) -> RenderPassStencilAttachmentDescriptor {
        unsafe { FromRaw::from_raw(self.0.stencilAttachment()).unwrap() }
    }

    pub fn set_stencil_attachment(&mut self, attachment: RenderPassStencilAttachmentDescriptor) {
        unsafe { self.0.setStencilAttachment(attachment.into_raw()) }
    }

    #[cfg(target_os = "macos")]
    pub fn render_target_array_length(&self) -> usize {
        unsafe { self.0.renderTargetArrayLength() as usize }
    }

    #[cfg(target_os = "macos")]
    pub fn set_render_target_array_length(&mut self, target_array_length: usize) {
        unsafe { self.0.setRenderTargetArrayLength(target_array_length as NSUInteger) }
    }

    /// Attempt to downcast this descriptor to a `RenderPassDepthAttachmentDescriptor`. If `self`
    /// is not a `RenderPassDepthAttachmentDescriptor`, then `self` will be returned in the `Err`
    /// branch.
    pub fn downcast_to_depth_descriptor(self) -> Result<RenderPassDepthAttachmentDescriptor, Self> {
        match FromRaw::from_raw(self.0) {
            Ok(descriptor) => Ok(descriptor),
            Err(_) => Err(self),
        }
    }

    /// Attempt to downcast this descriptor to a `RenderPassColorAttachmentDescriptor`. If `self`
    /// is not a `RenderPassColorAttachmentDescriptor`, then `self` will be returned in the `Err`
    /// branch.
    pub fn downcast_to_color_descriptor(self) -> Result<RenderPassColorAttachmentDescriptor, Self> {
        match FromRaw::from_raw(self.0) {
            Ok(descriptor) => Ok(descriptor),
            Err(_) => Err(self),
        }
    }

    /// Attempt to downcast this descriptor to a `RenderPassStencilAttachmentDescriptor`. If `self`
    /// is not a `RenderPassStencilAttachmentDescriptor`, then `self` will be returned in the `Err`
    /// branch.
    pub fn downcast_to_stencil_descriptor(
        self)
        -> Result<RenderPassStencilAttachmentDescriptor, Self> {
        match FromRaw::from_raw(self.0) {
            Ok(descriptor) => Ok(descriptor),
            Err(_) => Err(self),
        }
    }
}

impl Clone for RenderPassDescriptor {
    fn clone(&self) -> Self {
        let cloned = unsafe { self.0.copy() };
        FromRaw::from_raw(cloned).unwrap()
    }
}

impl_from_into_raw!(RenderPassDescriptor, of class "MTLRenderPassDescriptor");
