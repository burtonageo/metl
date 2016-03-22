use cocoa::base::id;
use std::borrow::Cow;
use {CompareFunction, Device, StencilDescriptor};

pub struct DepthStencilState(id);

impl DepthStencilState {
    pub fn device(&self) -> &Device {
        unimplemented!();
    }

    pub fn label(&self) -> Cow<str> {
        unimplemented!();
    }

    pub fn set_label(&mut self, label: &str) {
        unimplemented!();
    }
}

impl_from_into_raw!(DepthStencilState, of protocol "MTLDepthStencilState");

pub struct DepthStencilStateDescriptor(id);

impl DepthStencilStateDescriptor {
    pub fn depth_compare_function(&self) -> CompareFunction {
        unimplemented!();
    }

    pub fn set_depth_compare_function(&mut self, compare_function: CompareFunction) {
        unimplemented!();
    }

    pub fn depth_write_enabled(&self) -> bool {
        unimplemented!();
    }

    pub fn set_depth_write_enabled(&mut self, depth_write_enabled: bool) {
        unimplemented!();
    }

    pub fn back_face_stencil(&self) -> StencilDescriptor {
        unimplemented!();
    }
    
    pub fn set_back_face_stencil(&mut self, back_face_stencil: StencilDescriptor) {
        unimplemented!();
    }

    pub fn front_face_stencil(&self) -> StencilDescriptor {
        unimplemented!();
    }

    pub fn set_front_face_stencil(&mut self, front_face_stencil: StencilDescriptor) {
        unimplemented!();
    }

    pub fn label(&self) -> Cow<str> {
        unimplemented!();
    }
    
    pub fn set_label(&mut self, label: &str) {
        unimplemented!();
    }
}
