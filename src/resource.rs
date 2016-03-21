#![allow(unused_imports)]

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use std::borrow::Cow;
use std::ffi::CStr;
use std::mem;
use sys::{MTLCPUCacheMode, MTLPurgeableState, MTLResource, MTLResourceCPUCacheModeDefaultCache,
          MTLResourceCPUCacheModeShift, MTLResourceCPUCacheModeWriteCombined,
          MTLResourceOptionCPUCacheModeDefault, MTLResourceOptionCPUCacheModeWriteCombined,
          MTLResourceOptions, MTLResourceStorageModePrivate, MTLResourceStorageModeShared,
          MTLResourceStorageModeShift, MTLStorageMode};
use Device;

pub struct Resource(id);

impl Resource {
    pub fn cpu_cache_mode(self) -> CpuCacheMode {
        unsafe { self.0.cpuCacheMode().into() }
    }

    pub fn storage_mode(self) -> StorageMode {
        unsafe { self.0.storageMode().into() }
    }

    pub fn device(&self) -> &Device {
        unsafe { mem::transmute(self.0.device()) }
    }

    pub fn label(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.0.label().UTF8String()).to_string_lossy() }
    }

    pub fn set_label(&mut self, label: &str) {
        unsafe { self.0.setLabel(NSString::alloc(nil).init_str(label)) }
    }

    pub fn set_purgeable_state(self, state: PurgeableState) -> PurgeableState {
        unsafe { self.0.setPurgeableState(state.into()).into() }
    }
}

impl_from_into_raw!(Resource, of protocol "MTLResource");

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum StorageMode: MTLStorageMode {
        Shared => MTLStorageModeShared,
        Private => MTLStorageModePrivate
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum CpuCacheMode: MTLCPUCacheMode {
        DefaultCache => MTLCPUCacheModeDefaultCache,
        WriteCombined => MTLCPUCacheModeWriteCombined
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum PurgeableState: MTLPurgeableState {
       KeepCurrent => MTLPurgeableStateKeepCurrent,
       NonVolatile => MTLPurgeableStateNonVolatile,
       Volatile => MTLPurgeableStateVolatile,
       Empty => MTLPurgeableStateEmpty
    }
}
