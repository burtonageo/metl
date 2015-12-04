#![allow(unused_variables)]

use block::Block;
use cocoa::base::{BOOL, id};
use cocoa::foundation::NSUInteger;
use libc::c_void;
use {MTLFeatureSet, MTLPipelineOption, MTLSize};

/// The `MTLDevice` protocol defines the interface to a single graphics processor (GPU). You use
/// an object that conforms to this protocol to query the capabilities of the processor and to
/// allocate objects used to access those capabilities.
///
/// Your app does not define classes that implement this protocol; it is used by Metal to provide
/// a device object to your app. To obtain a system device, call the `MTLCreateSystemDefaultDevice`
/// function or select a result from the `MTLCopyAllDevices` function.
///
/// Most objects in Metal that perform graphics rendering and computational work are associated
/// directly with a specific device. For example, texture objects are created by a device object
/// and can be used only with that device. Most methods on a `MTLDevice` object create non-transient
/// objects, including command queues, resources (such as buffers and textures), and pipeline states.
/// These objects can be expensive to create and you are encouraged to create them soon after your
/// app launches and reuse them throughout the lifetime of your app. Avoid creating these objects
/// in performance sensitive code.
pub trait MTLDevice {
    /// A Boolean value that indicates whether a device supports a packed depth/stencil buffer. (read-only)
    ///
    /// # Discussion
    ///
    /// If the value is `YES`, the device supports the MTLPixelFormatDepth24Unorm_Stencil8 pixel format.
    unsafe fn depth24Stencil8PixelFormatSupported(self) -> BOOL;

    /// A Boolean value that indicates whether a device is configured as headless. (read-only)
    ///
    /// # Discussion
    ///
    /// If the value is `YES`, the device can not and does not have any displays attached.
    unsafe fn headless(self) -> BOOL;

    /// Boolean value that indicates whether a device is low-power. (read-only)
    ///
    /// # Discussion
    ///
    /// On multi-GPU systems that support automatic graphics switching, this property will return `YES`
    /// for the lower-power GPU.
    unsafe fn lowPower(self) -> BOOL;

    /// The maximum number of threads along each dimension of a threadgroup. (read-only)
    ///
    /// # Discussion
    ///
    /// This is the maximum compute grid size (width, height, depth) supported by the device.
    ///
    /// For more information on the specific capabilities of each feature set, see [Metal Programming Guide](
    /// https://developer.apple.com/library/prerelease/ios/documentation/
    ///     Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/
    ///     Introduction.html#//apple_ref/doc/uid/TP40014221
    /// ).
    unsafe fn maxThreadsPerGroup(self) -> MTLSize;

    /// A string that identifies the device. (read-only)
    ///
    /// # Discussion
    ///
    /// When this object is viewed in the Xcode debugger, its name is used to label the device.
    /// The name should only be used for informational purposes.
    unsafe fn name(self) -> id;

    /// Determines whether a device implements a particular feature set.
    ///
    /// # Parameters
    ///
    /// `featureSet` - A value that indicates the feature set you are interested in.
    ///
    /// # Return Value
    ///
    /// `YES` if the feature set is supported by the device; otherwise `NO`.
    ///
    /// # Discussion
    ///
    /// For more information on the specific capabilities of each feature set, see [Metal Programming Guide](
    /// https://developer.apple.com/library/prerelease/ios/documentation/
    ///     Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/
    ///     Introduction.html#//apple_ref/doc/uid/TP40014221
    /// ).
    unsafe fn supportsFeatureSet(self, featureSet: MTLFeatureSet) -> BOOL;
    
    /// Determines whether a device supports a given texture sample count.
    ///
    /// # Parameters
    ///
    /// `textureSampleCount` - The sample count you are interested in.
    ///
    /// # Return Value
    ///
    /// `YES` if the sample count is supported by the device; otherwise `NO`.
    ///
    /// # Discussion
    ///
    /// For more information on the specific capabilities of each feature set, see [Metal Programming Guide](
    /// https://developer.apple.com/library/prerelease/ios/documentation/
    ///     Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/
    ///     Introduction.html#//apple_ref/doc/uid/TP40014221
    /// ).
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

pub type MTLNewLibraryCompletionHandler = Block<(id,), id>;
pub type MTLNewRenderPipleineStateCompletionHandler = Block<(id, id), ()>;
pub type MTLNewRenderPipelineStateWithReflectionCompletionHandler = Block<(id, id, id), ()>;
pub type MTLNewComputePipelineStateCompletionHandler = Block<(id, id), ()>;
pub type MTLNewComputePipelineStateWithReflectionCompletionHandler = Block<(id, id, id), ()>;
