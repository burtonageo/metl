use cocoa::base::{BOOL, YES, id, nil};
use cocoa::foundation::{NSString, NSUInteger};
use sys::{MTLSamplerAddressMode, MTLSamplerDescriptor, MTLSamplerMinMagFilter,
          MTLSamplerMipFilter, MTLSamplerState};
use std::ffi::CStr;
use std::mem;
use {CompareFunction, Device, FromRaw, StrongPtr};

pub struct SamplerState(id);

impl SamplerState {
    pub fn device(&self) -> &Device {
        unsafe { mem::transmute(MTLSamplerState::device(self.0)) }
    }

    pub fn label(&self) -> &str {
        unsafe {
            CStr::from_ptr(MTLSamplerState::label(self.0).UTF8String()).to_str().unwrap_or(&"")
        }
    }
}

impl_from_into_raw!(SamplerState, of protocol "MTLSamplerState");

pub struct SamplerDescriptor(StrongPtr);

impl SamplerDescriptor {
    pub fn new() -> Self {
        unsafe { FromRaw::from_raw(MTLSamplerDescriptor::new(nil)).unwrap() }
    }

    pub fn r_address_mode(&self) -> SamplerAddressMode {
        unsafe { self.0.rAddressMode().into() }
    }

    pub fn set_r_address_mode(&mut self, address_mode: SamplerAddressMode) {
        unsafe { self.0.setRAddressMode(address_mode.into()) }
    }

    pub fn s_address_mode(&self) -> SamplerAddressMode {
        unsafe { self.0.sAddressMode().into() }
    }

    pub fn set_s_address_mode(&mut self, address_mode: SamplerAddressMode) {
        unsafe { self.0.setSAddressMode(address_mode.into()) }
    }

    pub fn t_address_mode(&self) -> SamplerAddressMode {
        unsafe { self.0.tAddressMode().into() }
    }

    pub fn set_t_address_mode(&mut self, address_mode: SamplerAddressMode) {
        unsafe { self.0.setTAddressMode(address_mode.into()) }
    }

    pub fn min_filter(&self) -> SamplerMinMagFilter {
        unsafe { self.0.minFilter().into() }
    }

    pub fn set_min_filter(&mut self, min_filter: SamplerMinMagFilter) {
        unsafe { self.0.setMinFilter(min_filter.into()) }
    }

    pub fn mag_filter(&self) -> SamplerMinMagFilter {
        unsafe { self.0.magFilter().into() }
    }

    pub fn set_mag_filter(&mut self, mag_filter: SamplerMinMagFilter) {
        unsafe { self.0.setMagFilter(mag_filter.into()) }
    }

    pub fn mip_filter(&self) -> SamplerMipFilter {
        unsafe { self.0.mipFilter().into() }
    }

    pub fn set_mip_filter(&mut self, mip_filter: SamplerMipFilter) {
        unsafe { self.0.setMipFilter(mip_filter.into()) }
    }

    pub fn lod_min_clamp(&self) -> f32 {
        unsafe { self.0.lodMinClamp() }
    }

    pub fn set_lod_min_clamp(&mut self, lod_min_clamp: f32) {
        unsafe { self.0.setLodMinClamp(lod_min_clamp) }
    }

    pub fn lod_max_clamp(&self) -> f32 {
        unsafe { self.0.lodMaxClamp() }
    }

    pub fn set_lod_max_clamp(&mut self, lod_min_clamp: f32) {
        unsafe { self.0.setLodMaxClamp(lod_min_clamp) }
    }

    pub fn lod_average(&self) -> bool {
        unsafe { self.0.lodAverage() == YES }
    }

    pub fn set_lod_average(&mut self, lod_average: bool) {
        unsafe { self.0.setLodAverage(lod_average as BOOL) }
    }

    pub fn max_anisotropy(&self) -> usize {
        unsafe { self.0.maxAnisotropy() as usize }
    }

    pub fn set_max_anisotropy(&mut self, anisotropy: usize) {
        unsafe { self.0.setMaxAnisotropy(anisotropy as NSUInteger) }
    }

    pub fn has_normalized_coordinates(&self) -> bool {
        unsafe { self.0.normalizedCoordinates() == YES }
    }

    pub fn set_has_normalized_coordinates(&mut self, normalized_coordinates: bool) {
        unsafe { self.0.setNormalizedCoordinates(normalized_coordinates as BOOL) }
    }

    pub fn compare_function(&self) -> CompareFunction {
        unsafe { self.0.compareFunction().into() }
    }

    pub fn set_compare_function(&mut self, compare_function: CompareFunction) {
        unsafe { self.0.setCompareFunction(compare_function.into()) }
    }

    pub fn label(&self) -> &str {
        unsafe {
            CStr::from_ptr(MTLSamplerDescriptor::label(*self.0).UTF8String())
                .to_str()
                .unwrap_or(&"")
        }
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
