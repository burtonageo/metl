use block::Block;
use cocoa::base::{BOOL, id};
use cocoa::foundation::{NSString, NSUInteger};
//use libc::c_void;
use {MTLFeatureSet, MTLSize};

pub type MTLNewLibraryCompletionHandler = Block<(id,), id>;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub trait MTLDevice {
    unsafe fn depth24Stencil8PixelFormatSupported(self) -> BOOL;
    unsafe fn headless(self) -> BOOL;
    unsafe fn lowPower(self) -> BOOL;
    unsafe fn maxThreadsPerGroup(self) -> MTLSize;
    unsafe fn name(self) -> id;
    unsafe fn supportsFeatureSet(self, featureSet: MTLFeatureSet) -> BOOL;
    unsafe fn supportsTextureSampleCount(self, textureSampleCount: NSUInteger) -> BOOL;

    unsafe fn newDefaultLibrary(self) -> id;
    unsafe fn newLibraryWithFile_Error(self, filePath: id, error: id) -> id;
    unsafe fn newLibraryWithSource_Options_CompletionHandler(self,
                                                             source: id,
                                                             options: id,
                                                             completionHandler: MTLNewLibraryCompletionHandler);
    unsafe fn newLibraryWithSource_Options_Error(self, source: id, options: id, error: id);
    unsafe fn newLibraryWithData_Error(self, data: id, error: id);

    unsafe fn newCommandQueue(self) -> id;
    unsafe fn newCommandQueueWithMaxCommandBufferCount(self, maxCommandBufferCount: NSUInteger) -> id;
}
