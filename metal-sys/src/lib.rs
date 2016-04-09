#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![crate_type = "rlib"]

#[macro_use]
extern crate bitflags;
extern crate block;
extern crate cocoa;
extern crate core_foundation;
extern crate core_graphics;
extern crate libc;
#[macro_use]
extern crate objc;

mod classes;
mod constants;
mod functions;
mod protocols;
mod types;

pub use classes::{MTLArgument, MTLArgumentAccess, MTLArgumentType, MTLArrayType,
                  MTLCompileOptions, MTLDataType, MTLDepthStencilDescriptor, MTLLoadAction,
                  MTLMultisampleDepthResolveFilter, MTLRenderPassAttachmentDescriptor,
                  MTLRenderPassColorAttachmentDescriptor,
                  MTLRenderPassColorAttachmentDescriptorArray,
                  MTLRenderPassDepthAttachmentDescriptor, MTLRenderPassDescriptor,
                  MTLRenderPassStencilAttachmentDescriptor, MTLSamplerAddressMode,
                  MTLSamplerDescriptor, MTLSamplerMinMagFilter, MTLSamplerMipFilter,
                  MTLStoreAction, MTLStructMember, MTLStructType, MTLTextureDescriptor,
                  MTLVertexAttribute};

pub use constants::{MTLCommandBufferStatus, MTLCompareFunction, MTLFeatureSet, MTLPipelineOption,
                    MTLPixelFormat};

pub use functions::{MTLClearColorMake, MTLCopyAllDevices, MTLCreateSystemDefaultDevice,
                    MTLOriginMake, MTLRegionMake1D, MTLRegionMake2D, MTLRegionMake3D, MTLSizeMake};

pub use protocols::{MTLCPUCacheMode, MTLCommandBuffer, MTLCommandQueue, MTLComputePipelineState,
                    MTLCullMode, MTLDepthClipMode, MTLDepthStencilState, MTLDevice, MTLDrawable,
                    MTLFunction, MTLFunctionType, MTLIndexType, MTLLanguageVersion, MTLLibrary,
                    MTLLibraryError, MTLLibraryErrorDomain,
                    MTLNewComputePipelineStateCompletionHandler,
                    MTLNewComputePipelineStateWithReflectionCompletionHandler,
                    MTLNewLibraryCompletionHandler,
                    MTLNewRenderPipelineStateWithReflectionCompletionHandler,
                    MTLNewRenderPipleineStateCompletionHandler, MTLPrimitiveType,
                    MTLPurgeableState, MTLRenderCommandEncoder, MTLRenderPipelineError,
                    MTLRenderPipelineErrorDomain, MTLResource,
                    MTLResourceCPUCacheModeDefaultCache, MTLResourceCPUCacheModeShift,
                    MTLResourceCPUCacheModeWriteCombined, MTLResourceOptionCPUCacheModeDefault,
                    MTLResourceOptionCPUCacheModeWriteCombined, MTLResourceOptions,
                    MTLResourceStorageModePrivate, MTLResourceStorageModeShared,
                    MTLResourceStorageModeShift, MTLSamplerState, MTLStorageMode, MTLTexture,
                    MTLTextureType, MTLTextureUsage, MTLTriangleFillMode,
                    MTLVisibilityResultMode, MTLWinding};

pub use types::{MTLClearColor, MTLDispatchThreadgroupsIndirectArguments,
                MTLDrawIndexedPrimitivesIndirectArguments, MTLDrawPrimitivesIndirectArguments,
                MTLOrigin, MTLRegion, MTLScissorRect, MTLSize, MTLViewport};
