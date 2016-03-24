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
                  MTLCompileOptions, MTLDataType, MTLLoadAction, MTLMultisampleDepthResolveFilter,
                  MTLRenderPassAttachmentDescriptor, MTLRenderPassColorAttachmentDescriptor,
                  MTLRenderPassColorAttachmentDescriptorArray,
                  MTLRenderPassDepthAttachmentDescriptor, MTLRenderPassDescriptor,
                  MTLRenderPassStencilAttachmentDescriptor, MTLStoreAction, MTLStructMember,
                  MTLStructType, MTLVertexAttribute, MTLDepthStencilStateDescriptor};

pub use constants::{MTLCommandBufferStatus, MTLCompareFunction, MTLFeatureSet, MTLPipelineOption,
                    MTLPixelFormat};

pub use functions::{MTLClearColorMake, MTLCopyAllDevices, MTLCreateSystemDefaultDevice,
                    MTLOriginMake, MTLRegionMake1D, MTLRegionMake2D, MTLRegionMake3D, MTLSizeMake};

pub use protocols::{MTLCommandBuffer, MTLCommandQueue, MTLComputePipelineState, MTLDevice,
                    MTLDrawable, MTLFunction, MTLFunctionType, MTLLanguageVersion, MTLLibrary,
                    MTLLibraryError, MTLLibraryErrorDomain,
                    MTLNewComputePipelineStateCompletionHandler,
                    MTLNewComputePipelineStateWithReflectionCompletionHandler,
                    MTLNewLibraryCompletionHandler,
                    MTLNewRenderPipelineStateWithReflectionCompletionHandler,
                    MTLNewRenderPipleineStateCompletionHandler, MTLRenderPipelineError,
                    MTLRenderPipelineErrorDomain, MTLTexture, MTLTextureType, MTLTextureUsage,
                    MTLResource, MTLCPUCacheMode, MTLStorageMode, MTLResourceOptions,
                    MTLPurgeableState, MTLResourceStorageModeShift, MTLResourceCPUCacheModeShift,
                    MTLResourceCPUCacheModeDefaultCache,MTLResourceCPUCacheModeWriteCombined,
                    MTLResourceStorageModeShared,MTLResourceStorageModePrivate,
                    MTLResourceOptionCPUCacheModeDefault,MTLResourceOptionCPUCacheModeWriteCombined};

pub use types::{MTLClearColor, MTLDispatchThreadgroupsIndirectArguments,
                MTLDrawIndexedPrimitivesIndirectArguments, MTLDrawPrimitivesIndirectArguments,
                MTLOrigin, MTLRegion, MTLScissorRect, MTLSize, MTLViewport};
