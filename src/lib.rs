extern crate metal_sys;

pub mod sys {
    pub use metal_sys::*;
}

pub type ClearColor = sys::MTLClearColor;
pub type Origin = sys::MTLOrigin;
pub type Region = sys::MTLRegion;
pub type ScissorRect = sys::MTLScissorRect;
pub type Size = sys::MTLSize;
pub type Viewport = sys::MTLViewport;
pub type DrawPrimitivesIndirectArguments = sys::MTLDrawPrimitivesIndirectArguments;
pub type DrawIndexedPrimitivesIndirectArguments = sys::MTLDrawIndexedPrimitivesIndirectArguments;
pub type DispatchThreadgroupsIndirectArguments = sys::MTLDispatchThreadgroupsIndirectArguments;
