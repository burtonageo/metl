use cocoa::base::{BOOL, id, nil, YES};
use cocoa::foundation::NSString;
use std::borrow::Cow;
use std::ffi::CStr;
use sys::MTLDepthStencilStateDescriptor;
use {CompareFunction, Device, FromRaw, IntoRaw, StencilDescriptor};

pub struct DepthStencilState(id);

impl DepthStencilState {
    pub fn device(&self) -> &Device {
        unimplemented!();
    }

    pub fn label(&self) -> Cow<str> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    pub fn set_label(&mut self, label: &str) {
        unimplemented!();
    }
}

impl_from_into_raw!(DepthStencilState, of protocol "MTLDepthStencilState");

pub struct DepthStencilStateDescriptor(id);

impl DepthStencilStateDescriptor {
    pub fn depth_compare_function(&self) -> CompareFunction {
        unsafe { self.0.depthCompareFunction().into() }
    }

    pub fn set_depth_compare_function(&mut self, compare_function: CompareFunction) {
        unsafe { self.0.setDepthCompareFunction(compare_function.into()) }
    }

    pub fn depth_write_enabled(&self) -> bool {
        unsafe { self.0.depthWriteEnabled() == YES }
    }

    pub fn set_depth_write_enabled(&mut self, depth_write_enabled: bool) {
        unsafe { self.0.setDepthWriteEnabled(depth_write_enabled as BOOL)}
    }

    pub fn back_face_stencil(&self) -> Option<StencilDescriptor> {
        unsafe { FromRaw::from_raw(self.0.backFaceStencil()).ok() }
    }

    pub fn set_back_face_stencil(&mut self, back_face_stencil: StencilDescriptor) {
        unsafe { self.0.setBackFaceStencil(back_face_stencil.into_raw()) }
    }

    pub fn clear_back_face_stencil(&mut self) {
        unsafe { self.0.setBackFaceStencil(nil) }
    }

    pub fn front_face_stencil(&self) -> Option<StencilDescriptor> {
        unsafe { FromRaw::from_raw(self.0.frontFaceStencil()).ok() }
    }

    pub fn set_front_face_stencil(&mut self, front_face_stencil: StencilDescriptor) {
        unsafe { self.0.setFrontFaceStencil(front_face_stencil.into_raw()) }
    }

    pub fn clear_front_face_stencil(&mut self) {
        unsafe { self.0.setFrontFaceStencil(nil) }
    }

    pub fn label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.label().UTF8String()).to_string_lossy() }
    }

    pub fn set_label(&mut self, label: &str) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label)) }
    }
}

impl_from_into_raw!(DepthStencilStateDescriptor, of class "MTLDepthStencilStateDescriptor");
