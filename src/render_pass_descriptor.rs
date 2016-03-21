use cocoa::base::{id, nil};
use cocoa::foundation::NSUInteger;
use sys::MTLRenderPassDescriptor;
use {FromRaw, IntoRaw, RenderPassDepthAttachmentDescriptor, RenderPassColorAttachmentDescriptorArray};

pub struct RenderPassDescriptor(id);

#[allow(unused_variables)]
impl RenderPassDescriptor {
    pub fn new() -> Self {
        unsafe { FromRaw::from_raw(MTLRenderPassDescriptor::renderPassDescriptor(nil)).unwrap() }
    }

    pub fn color_attachments(&self) -> RenderPassColorAttachmentDescriptorArray {
        unsafe { FromRaw::from_raw(self.0.colorAttachments()).unwrap() }
    }

    pub fn set_color_attachments(&self, attachments: RenderPassColorAttachmentDescriptorArray) {
        unsafe { self.0.setColorAttachments(attachments.into_raw()) }
    }

    pub fn depth_attachment(&self) -> RenderPassDepthAttachmentDescriptor {
        unsafe { FromRaw::from_raw(self.0.depthAttachment()).unwrap() }
    }

    pub fn set_depth_attachment(&self, attachment: RenderPassDepthAttachmentDescriptor) {
        unsafe { self.0.setDepthAttachment(attachment.into_raw()) }
    }

    pub fn stencil_attachment(&self) -> ! {
        unimplemented!();
    }

    pub fn set_stencil_attachment(&self, attachment: ()) {
        unimplemented!();
    }

    #[cfg(target_os = "macos")]
    pub fn render_target_array_length(self) -> usize {
        unsafe { self.0.renderTargetArrayLength() as usize }
    }

    #[cfg(target_os = "macos")]
    pub fn set_render_target_array_length(self, target_array_length: usize) {
        unsafe { self.0.setRenderTargetArrayLength(target_array_length as NSUInteger) }
    }
}

impl Clone for RenderPassDescriptor {
    fn clone(&self) -> Self {
        let cloned = unsafe { self.0.copy() };
        FromRaw::from_raw(cloned).unwrap()
    }
}

impl_from_into_raw!(RenderPassDescriptor, of class "MTLRenderPassDescriptor");
