//! The Metal framework defines data types that are used in multiple places throughout the framework.
//!
//! Taken from https://developer.apple.com/library/ios/documentation/Metal/Reference/MetalDataTypes_Ref/index.html

use cocoa::foundation::NSUInteger;
use libc::uint32_t;

/// An RGBA value used for a color pixel.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct MTLClearColor {
    /// The red color channel.
    pub red: f64,

    /// The green color channel.
    pub green: f64,

    /// The blue color channel.
    pub blue: f64,

    /// The alpha color channel.
    pub alpha: f64
}

/// A location of a pixel in an image or texture, relative to the upper-left corner which is (0,0).
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MTLOrigin {
    /// The x coordinate of the location.
    pub x: NSUInteger,

    /// The y coordinate of the location.
    pub y: NSUInteger,

    /// The z coordinate of the location.
    pub z: NSUInteger
}

/// A rectangular block of pixels in an image or texture, defined by its upper-left corner and its size.
#[repr(C)]
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct MTLRegion {
    /// The location of the upper-left corner of the block.
    pub origin: MTLOrigin,

    /// The size of the block.
    pub size: MTLSize
}

/// A rectangle for the scissor fragment test.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MTLScissorRect {
    /// The x window coordinate of the upper-left corner of the scissor rectangle.
    pub x: NSUInteger,

    /// The y window coordinate of the upper-left corner of the scissor rectangle.
    pub y: NSUInteger,

    /// The width of the scissor rectangle, in pixels.
    pub width: NSUInteger,

    /// The height of the scissor rectangle, in pixels.
    pub height: NSUInteger
}

/// A set of dimensions to declare the size of an object, such as an image, texture, threadgroup, or grid.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MTLSize {
    /// The x dimension of the object size.
    pub width: NSUInteger,

    /// The y dimension of the object size.
    pub height: NSUInteger,

    /// The z dimension of the object size.
    pub depth: NSUInteger
}

/// A 3D rectangular region for the viewport clipping.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct MTLViewport {
    /// The x coordinate of the upper-left corner of the viewport.
    pub originX: f64,

    /// The y coordinate of the upper-left corner of the viewport.
    pub originY: f64,

    /// The width of the viewport, in pixels.
    pub width: f64,

    /// The height of the viewport, in pixels.
    pub height: f64,

    /// The z coordinate of the near clipping plane of the viewport.
    pub znear: f64,

    /// The z coordinate of the far clipping plane of the viewport.
    pub zfar: f64
}

/// The data layout required for drawing primitives via indirect buffer calls.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MTLDrawPrimitivesIndirectArguments {
    /// The number of indices to draw.
    pub vertexCount: uint32_t,

    /// The number of instances to draw.
    pub instanceCount: uint32_t,

    /// The first index to draw.
    pub vertexStart: uint32_t,

    /// The first instance to draw.
    pub baseInstance: uint32_t
}

/// The data layout required for drawing indexed primitives via indirect buffer calls.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
    /// For each instance, the number of indices to read from the index buffer.
    pub indexCount: uint32_t,

    /// The number of instances to draw.
    pub instanceCount: uint32_t,

    /// The first index to draw.
    pub indexStart: uint32_t,

    /// The first vertex to draw.
    pub baseVertex: uint32_t,

    /// The first instance to draw.
    pub baseInstance: uint32_t
}

/// The data layout required for dispatching threadgroups via indirect buffer calls.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    /// The number of threadgroups for the grid, in each dimension.
    pub threadgroupsPerGrid: [uint32_t; 3]
}
