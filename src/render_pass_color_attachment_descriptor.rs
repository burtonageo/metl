use cocoa::base::id;
use std::mem;
use std::ops::Deref;
use sys::MTLRenderPassColorAttachmentDescriptor;
use {ClearColor, FromRaw, RenderPassAttachmentDescriptor};

pub struct RenderPassColorAttachmentDescriptor(id);

impl RenderPassColorAttachmentDescriptor {
    pub fn clear_color(&self) -> ClearColor {
        unsafe { self.0.clearColor().into() }
    }
}

impl_from_into_raw!(RenderPassColorAttachmentDescriptor, of class "MTLRenderPassColorAttachmentDescriptor");

impl Deref for RenderPassColorAttachmentDescriptor {
    type Target = RenderPassAttachmentDescriptor;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl Clone for RenderPassColorAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe { FromRaw::from_raw(self.0.copy()).unwrap() }
    }
}
