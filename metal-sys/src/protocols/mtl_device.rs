use block::Block;
use cocoa::base::{BOOL, id};
use cocoa::foundation::{NSString, NSUInteger};
use libc::c_void;
use {MTLFeatureSet, MTLPipelineOption, MTLSize};

pub type MTLNewLibraryCompletionHandler = Block<(id,), id>;
pub type MTLNewRenderPipleineStateCompletionHandler = Block<(id, id), ()>;
pub type MTLNewRenderPipelineStateWithReflectionCompletionHandler = Block<(id, id, id), ()>;
pub type MTLNewComputePipelineStateCompletionHandler = Block<(id, id), ()>;
pub type MTLNewComputePipelineStateWithReflectionCompletionHandler = Block<(id, id, id), ()>;

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
    unsafe fn newLibraryWithFile_error(self, filePath: id, error: id) -> id;
    unsafe fn newLibraryWithSource_options_completionHandler(self,
                                                             source: id,
                                                             options: id,
                                                             completionHandler: MTLNewLibraryCompletionHandler);
    unsafe fn newLibraryWithSource_options_error(self, source: id, options: id, error: id);
    unsafe fn newLibraryWithData_error(self, data: id, error: id);

    unsafe fn newCommandQueue(self) -> id;
    unsafe fn newCommandQueueWithMaxCommandBufferCount(self, maxCommandBufferCount: NSUInteger) -> id;

    unsafe fn newBufferWithLength_Options(self, length: NSUInteger, options: id) -> id;
    unsafe fn newBufferWithBytes_Length_Options(self, pointer: *mut c_void, length: NSUInteger, options: id) -> id;
    unsafe fn newBufferWithBytesNoCopy_length_options_deallocator(self,
                                                                  pointer: *mut c_void,
                                                                  length: NSUInteger,
                                                                  deallocator: Block<(*mut c_void, NSUInteger), ()>);

    unsafe fn newTextureWithDescriptor(self, descriptor: id) -> id;
    unsafe fn newSamplerStateWithDescriptor(self, descriptor: id) -> id;

    unsafe fn newStencilDepthStateWithDescriptor(self, descriptor: id) -> id;
    unsafe fn newRenderPipleineStateWithDescriptor_completionHandler(
                                                    self,
                                                    completionHandler: MTLNewRenderPipleineStateCompletionHandler);
    unsafe fn newRenderPipelineStateWithDescriptor_options_completionHandler(
                                        self,
                                        options: MTLPipelineOption,
                                        completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler);
    unsafe fn newRenderPipelineStateWithDescriptor_error(self, error: id);
    unsafe fn newRenderPipelineStateWithDescriptor_options_reflection(self,
                                                                      options: MTLPipelineOption,
                                                                      reflection: id,
                                                                      error: id);

    unsafe fn newComputePipelineStateWithFunction_completionHandler(
                                                        self,
                                                        function: id,
                                                        completionHandler: MTLNewComputePipelineStateCompletionHandler);
    unsafe fn newComputePipelineStateWithFunction_options_completionHandler(
                                        self,
                                        function: id,
                                        options: MTLPipelineOption,
                                        completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler);
    unsafe fn newComputePipelineStateWithFunction_error(self, function: id, error: id);
    unsafe fn newComputePipelineStateWithFunction_options_reflection_error(self,
                                                                           function: id,
                                                                           options: MTLPipelineOption,
                                                                           reflection: id,
                                                                           error: id);

    unsafe fn newComputePipelineStateWithDescriptor_options_completionHandler(self,
                                        descriptor: id,
                                        options: MTLPipelineOption,
                                        completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler);
    unsafe fn newComputePipelineStateWithDescriptor_options_reflection_error(self,
                                                                             descriptor: id,
                                                                             options: MTLPipelineOption,
                                                                             reflection: id,
                                                                             error: id);
}
