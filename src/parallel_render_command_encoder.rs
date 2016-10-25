use cocoa::base::id;
use std::mem;
use std::ops::{Deref, DerefMut};
use {CommandEncoder, StoreAction, RenderCommandEncoder};

#[derive(Debug)]
pub struct ParallelRenderCommandEncoder(id);

unsafe impl Send for ParallelRenderCommandEncoder {}

impl Deref for ParallelRenderCommandEncoder {
    type Target = CommandEncoder;
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(&self.0) }
    }
}

impl DerefMut for ParallelRenderCommandEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(&mut self.0) }
    }
}

impl ParallelRenderCommandEncoder {
    pub fn render_command_encoder(&self) -> RenderCommandEncoder {
        unimplemented!()
    }

    pub fn set_color_store_action_at(&self, store_action: StoreAction, index: usize) {
        unimplemented!()
    }

    pub fn set_depth_store_action(self, store_action: StoreAction) {
        unimplemented!()
    }

    pub fn set_stencil_store_action(self, store_action: StoreAction) {
        unimplemented!()
    }
}

impl_from_into_raw!(ParallelRenderCommandEncoder, of protocol "MTLRenderCommandEncoder");
