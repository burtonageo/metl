use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use sys::{MTLSamplerState, MTLSamplerDescriptor, MTLSamplerAddressMode, MTLSamplerMinMagFilter,
          MTLSamplerMipFilter};
use std::ffi::CStr;
use std::mem;
use {FromRaw, Device};

pub struct SamplerState(id);

impl SamplerState {
    pub fn device(&self) -> &Device {
        unsafe { mem::transmute(MTLSamplerState::device(self.0)) }
    }

    pub fn label(&self) -> &str {
        unsafe { CStr::from_ptr(MTLSamplerState::label(self.0).UTF8String()).to_str().unwrap_or(&"") }
    }
}

impl_from_into_raw!(SamplerState, of protocol "MTLSamplerState");

pub struct SamplerDescriptor(id);

impl SamplerDescriptor {
    pub fn new() -> Self {
        unsafe { FromRaw::from_raw(MTLSamplerDescriptor::new(nil)).unwrap() }
    }

    pub fn label(&self) -> &str {
        unsafe { CStr::from_ptr(MTLSamplerDescriptor::label(self.0).UTF8String()).to_str().unwrap_or(&"") }
    }

    pub fn set_label(&mut self, label: &str) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label)) }
    }
}

impl Clone for SamplerDescriptor {
    fn clone(&self) -> Self {
        unsafe { FromRaw::from_raw(self.0.copy()).unwrap() }
    }
}

impl_from_into_raw!(SamplerDescriptor, of class "MTLSamplerDescriptor");

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum SamplerAddressMode: MTLSamplerAddressMode {
        ClampToEdge => MTLSamplerAddressModeClampToEdge,
        Repeat => MTLSamplerAddressModeRepeat,
        MirrorRepeat => MTLSamplerAddressModeMirrorRepeat,
        ClampToZero => MTLSamplerAddressModeClampToZero
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum SamplerMinMagFilter: MTLSamplerMinMagFilter {
        Nearest => MTLSamplerMinMagFilterNearest,
        Linear => MTLSamplerMinMagFilterLinear
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum SamplerMipFilter: MTLSamplerMipFilter {
        NotMipmapped => MTLSamplerMipFilterNotMipmapped,
        Nearest => MTLSamplerMipFilterNearest,
        Linear => MTLSamplerMipFilterLinear
    }
}
