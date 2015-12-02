#![allow(unused_variables)]

use block::Block;
use cocoa::base::{BOOL, id};
use cocoa::foundation::NSUInteger;
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
                                                    descriptor: id,
                                                    completionHandler: MTLNewRenderPipleineStateCompletionHandler);
    unsafe fn newRenderPipelineStateWithDescriptor_options_completionHandler(
                                        self,
                                        descriptor: id,
                                        options: MTLPipelineOption,
                                        completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler);
    unsafe fn newRenderPipelineStateWithDescriptor_error(self, descriptor: id, error: id);
    unsafe fn newRenderPipelineStateWithDescriptor_options_reflection(self,
                                                                      descriptor: id,
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

impl MTLDevice for id {
    unsafe fn depth24Stencil8PixelFormatSupported(self) -> BOOL {
        msg_send![self, isDepth24Stencil8PixelFormatSupported]
    }

    unsafe fn headless(self) -> BOOL {
        msg_send![self, isHeadless]
    }

    unsafe fn lowPower(self) -> BOOL {
        msg_send![self, isLowPower]
    }

    unsafe fn maxThreadsPerGroup(self) -> MTLSize {
        msg_send![self, maxThreadsPerThreadgroup]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn supportsFeatureSet(self, featureSet: MTLFeatureSet) -> BOOL {
        msg_send![self, supportsFeatureSet:featureSet]
    }

    unsafe fn supportsTextureSampleCount(self, textureSampleCount: NSUInteger) -> BOOL {
        msg_send![self, supportsTextureSampleCount:textureSampleCount]
    }

    unsafe fn newDefaultLibrary(self) -> id {
        msg_send![self, newDefaultLibrary]
    }

    unsafe fn newLibraryWithFile_error(self, filePath: id, error: id) -> id {
        msg_send![self, newLibraryWithFile:filePath error:error]
    }

    unsafe fn newLibraryWithSource_options_completionHandler(self,
                                                             source: id,
                                                             options: id,
                                                             completionHandler: MTLNewLibraryCompletionHandler) {
        msg_send![self, newLibraryWithSource:source options:options completionHandler:completionHandler]
    }

    unsafe fn newLibraryWithSource_options_error(self, source: id, options: id, error: id) {
        msg_send![self, newLibraryWithSource:source options:options error:error]
    }

    unsafe fn newLibraryWithData_error(self, data: id, error: id) {
        msg_send![self, newLibraryWithData:data error:error]
    }


    unsafe fn newCommandQueue(self) -> id {
        msg_send![self, newCommandQueue]
    }

    unsafe fn newCommandQueueWithMaxCommandBufferCount(self, maxCommandBufferCount: NSUInteger) -> id {
        msg_send![self, newCommandQueueWithMaxCommandBufferCount:maxCommandBufferCount]
    }

    unsafe fn newBufferWithLength_Options(self, length: NSUInteger, options: id) -> id {
        msg_send![self, newBufferWithLength:length options:options]
    }

    unsafe fn newBufferWithBytes_Length_Options(self, pointer: *mut c_void, length: NSUInteger, options: id) -> id {
        msg_send![self, newBufferWithBytes:pointer length:length options:options]
    }

    unsafe fn newBufferWithBytesNoCopy_length_options_deallocator(self,
                                                                  pointer: *mut c_void,
                                                                  length: NSUInteger,
                                                                  deallocator: Block<(*mut c_void, NSUInteger), ()>) {
        msg_send![self, newBufferWithBytesNoCopy:pointer length:length deallocator:deallocator]
    }

    unsafe fn newTextureWithDescriptor(self, descriptor: id) -> id {
        msg_send![self, newTextureWithDescriptor:descriptor]
    }

    unsafe fn newSamplerStateWithDescriptor(self, descriptor: id) -> id {
        msg_send![self, newSamplerStateWithDescriptor:descriptor]
    }

    unsafe fn newStencilDepthStateWithDescriptor(self, descriptor: id) -> id {
        msg_send![self, newStencilDepthStateWithDescriptor:descriptor]
    }

    unsafe fn newRenderPipleineStateWithDescriptor_completionHandler(
                                                    self,
                                                    descriptor: id,
                                                    completionHandler: MTLNewRenderPipleineStateCompletionHandler) {
        msg_send![self, newRenderPipleineStateWithDescriptor:descriptor completionHandler:completionHandler]
    }

    unsafe fn newRenderPipelineStateWithDescriptor_options_completionHandler(
                                        self,
                                        descriptor: id,
                                        options: MTLPipelineOption,
                                        completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler) {
        msg_send![self, newRenderPipelineStateWithDescriptor:descriptor
                                                     options:options
                                           completionHandler:completionHandler]
    }

    unsafe fn newRenderPipelineStateWithDescriptor_error(self, descriptor: id, error: id) {
        msg_send![self, newRenderPipelineStateWithDescriptor:descriptor error:error]
    }

    unsafe fn newRenderPipelineStateWithDescriptor_options_reflection(self,
                                                                      descriptor: id,
                                                                      options: MTLPipelineOption,
                                                                      reflection: id,
                                                                      error: id) {
        msg_send![self, newRenderPipelineStateWithDescriptor:descriptor
                                                     options:options
                                                  reflection:reflection
                                                       error:error]
    }

    unsafe fn newComputePipelineStateWithFunction_completionHandler(
                  self,
                  function: id,
                  completionHandler: MTLNewComputePipelineStateCompletionHandler) {
        msg_send![self, newComputePipelineStateWithFunction:function completionHandler:completionHandler]
    }

    unsafe fn newComputePipelineStateWithFunction_options_completionHandler(
                   self,
                   function: id,
                   options: MTLPipelineOption,
                   completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler) {
        msg_send![self, newComputePipelineStateWithFunction:function
                                                    options:options
                                          completionHandler:completionHandler]
    }

    unsafe fn newComputePipelineStateWithFunction_error(self, function: id, error: id) {
        msg_send![self, newComputePipelineStateWithFunction:function error:error]
    }

    unsafe fn newComputePipelineStateWithFunction_options_reflection_error(self,
                                                                           function: id,
                                                                           options: MTLPipelineOption,
                                                                           reflection: id,
                                                                           error: id) {
        msg_send![self, newComputePipelineStateWithFunction:function options:options reflection:reflection error:error]
    }

    unsafe fn newComputePipelineStateWithDescriptor_options_completionHandler(
                   self,
                   descriptor: id,
                   options: MTLPipelineOption,
                   completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler) {
        msg_send![self, newComputePipelineStateWithDescriptor:descriptor
                                                      options:options
                                            completionHandler:completionHandler]
    }
    unsafe fn newComputePipelineStateWithDescriptor_options_reflection_error(self,
                                                                             descriptor: id,
                                                                             options: MTLPipelineOption,
                                                                             reflection: id,
                                                                             error: id) {
        msg_send![self, newComputePipelineStateWithDescriptor:descriptor
                                                      options:options
                                                   reflection:reflection
                                                        error:error]
    }
}
