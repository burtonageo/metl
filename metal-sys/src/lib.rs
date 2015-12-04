#![allow(non_snake_case)]
#![crate_type = "rlib"]

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

pub use constants::{MTLCompareFunction, MTLFeatureSet, MTLPipelineOption, MTLPixelFormat};

pub use functions::{MTLClearColorMake, MTLCopyAllDevices, MTLCreateSystemDefaultDevice, MTLOriginMake,
                    MTLRegionMake1D, MTLRegionMake2D, MTLRegionMake3D, MTLSizeMake};

pub use protocols::{MTLCommandBuffer, MTLCommandQueue, MTLDevice};

pub use types::{MTLClearColor, MTLDispatchThreadgroupsIndirectArguments, MTLDrawIndexedPrimitivesIndirectArguments,
                MTLDrawPrimitivesIndirectArguments, MTLOrigin, MTLRegion, MTLScissorRect, MTLSize, MTLViewport};
