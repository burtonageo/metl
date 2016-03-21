use cocoa::base::id;
use std::mem;
use std::ops::Deref;
use sys::MTLRenderPassStencilAttachmentDescriptor;
use RenderPassAttachmentDescriptor;

pub struct RenderPassStencilAttachmentDescriptor(id);

impl RenderPassStencilAttachmentDescriptor {
    pub fn clear_stencil(&self) -> u32 {
        unsafe { self.0.clearStencil() }
    }

    pub fn set_clear_stencil(&mut self, stencil: u32) {
        unsafe { self.0.setClearStencil(stencil) }
    }
}

impl Deref for RenderPassStencilAttachmentDescriptor {
    type Target = RenderPassAttachmentDescriptor;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl_from_into_raw!(RenderPassStencilAttachmentDescriptor,
                    of class "MTLRenderPassStencilAttachmentDescriptor");
