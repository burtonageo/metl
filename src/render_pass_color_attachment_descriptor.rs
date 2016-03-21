use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use std::mem;
use std::ops::Deref;
use sys::{MTLRenderPassColorAttachmentDescriptor, MTLRenderPassColorAttachmentDescriptorArray};
use {ClearColor, FromRaw, IntoRaw, RenderPassAttachmentDescriptor};

pub struct RenderPassColorAttachmentDescriptor(id);

impl RenderPassColorAttachmentDescriptor {
    pub fn clear_color(&self) -> ClearColor {
        unsafe { self.0.clearColor().into() }
    }
}

impl_from_into_raw!(RenderPassColorAttachmentDescriptor,
                    of class "MTLRenderPassColorAttachmentDescriptor");

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

pub struct RenderPassColorAttachmentDescriptorArray(id);

impl RenderPassColorAttachmentDescriptorArray {
    pub fn attachment_at_index(&self, index: usize) -> RenderPassColorAttachmentDescriptor {
        let attachment = unsafe { self.0.objectAtIndexedSubscript(index as NSUInteger) };
        FromRaw::from_raw(attachment).unwrap()
    }

    pub fn set_attachment_at_index(&self, attachment: RenderPassColorAttachmentDescriptor,
                                   index: usize) {
        unsafe { self.0.setObject_atIndexedSubscript(attachment.into_raw(), index as NSUInteger) }
    }
}

impl_from_into_raw!(RenderPassColorAttachmentDescriptorArray,
                    of class "RenderPassColorAttachmentDescriptorArray");
