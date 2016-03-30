use cocoa::base::{BOOL, YES, id, nil};
use cocoa::foundation::NSString;
use std::ffi::CStr;
use std::mem;
use sys::{MTLDepthStencilDescriptor, MTLDepthStencilState};
use {CompareFunction, Device, FromRaw, IntoRaw, StencilDescriptor, StrongPtr};

pub struct DepthStencilState(id);

impl DepthStencilState {
    pub fn device(&self) -> &Device {
        unsafe { mem::transmute(self.0.device()) }
    }

    pub fn label(&self) -> &str {
        unsafe {
            CStr::from_ptr(MTLDepthStencilState::label(self.0).UTF8String())
                .to_str()
                .unwrap_or(&"")
        }
    }
}

impl_from_into_raw!(DepthStencilState, of protocol "MTLDepthStencilState");

pub struct DepthStencilDescriptor(StrongPtr);

impl DepthStencilDescriptor {
    pub fn new() -> Self {
        unsafe { FromRaw::from_raw(MTLDepthStencilDescriptor::new(nil)).unwrap() }
    }

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
        unsafe { self.0.setDepthWriteEnabled(depth_write_enabled as BOOL) }
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

    pub fn label(&self) -> &str {
        unsafe {
            CStr::from_ptr(MTLDepthStencilState::label(*self.0).UTF8String())
                .to_str()
                .unwrap_or(&"")
        }
    }

    pub fn set_label(&mut self, label: &str) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label)) }
    }
}

impl Clone for DepthStencilDescriptor {
    fn clone(&self) -> Self {
        unsafe { FromRaw::from_raw(self.0.copy()).unwrap() }
    }
}

impl_from_into_raw!(DepthStencilDescriptor, of class "MTLDepthStencilDescriptor");
