#![allow(non_snake_case)]

extern crate cocoa;
extern crate core_foundation;
extern crate core_graphics;
extern crate libc;
extern crate objc;

mod classes;
mod functions;
mod protocols;
mod types;

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
