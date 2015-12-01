#![allow(non_snake_case)]
#![crate_type = "rlib"]

extern crate block;
extern crate cocoa;
extern crate core_foundation;
extern crate core_graphics;
extern crate libc;
extern crate objc;

mod classes;
mod constants;
mod functions;
mod protocols;
mod types;

pub use constants::{
    MTLFeatureSet,
    MTLCompareFunction,
    MTLPipelineOption,
    MTLPixelFormat,
};

pub use functions::{
    MTLCopyAllDevices,
    MTLCreateSystemDefaultDevice,
    MTLOriginMake,
    MTLRegionMake1D,
    MTLRegionMake2D,
    MTLRegionMake3D,
    MTLSizeMake,
    MTLClearColorMake
};

pub use types::{
    MTLClearColor,
    MTLOrigin,
    MTLRegion,
    MTLScissorRect,
    MTLSize,
    MTLViewport,
    MTLDrawPrimitivesIndirectArguments,
    MTLDrawIndexedPrimitivesIndirectArguments,
    MTLDispatchThreadgroupsIndirectArguments
};

