extern crate metal_sys;
extern crate cocoa;

use cocoa::foundation::NSUInteger;

mod device;

pub mod sys {
    pub use metal_sys::*;
}

pub use sys::MTLClearColor as ClearColor;
pub use sys::MTLOrigin as Origin;
pub use sys::MTLRegion as Region;
pub use sys::MTLScissorRect as ScissorRect;
pub use sys::MTLSize as Size;
pub use sys::MTLViewport as Viewport;
pub use sys::MTLDrawPrimitivesIndirectArguments as DrawPrimitivesIndirectArguments;
pub use sys::MTLDrawIndexedPrimitivesIndirectArguments as DrawIndexedPrimitivesIndirectArguments;
pub use sys::MTLDispatchThreadgroupsIndirectArguments as DispatchThreadgroupsIndirectArguments;

pub use device::{Device, DeviceError};

pub fn clear_color(red: f64, green: f64, blue: f64, alpha: f64) -> ClearColor {
    sys::MTLClearColorMake(red, green, blue, alpha)
}

pub fn origin(x: usize, y: usize, z: usize) -> Origin {
    sys::MTLOriginMake(x as NSUInteger, y as NSUInteger, z as NSUInteger)
}

pub fn region_1d(x: usize, width: usize) -> Region {
    sys::MTLRegionMake1D(x as NSUInteger, width as NSUInteger)
}

pub fn region_2d(x: usize, y: usize, width: usize, height: usize) -> Region {
    sys::MTLRegionMake2D(x as NSUInteger, y as NSUInteger, width as NSUInteger, height as NSUInteger)
}

pub fn region_3d(x: usize, y: usize, z: usize, width: usize, height: usize, depth: usize) -> Region {
    sys::MTLRegionMake3D(x as NSUInteger,
                         y as NSUInteger,
                         z  as NSUInteger,
                         width as NSUInteger,
                         height as NSUInteger,
                         depth as NSUInteger)
}

 pub fn size(width: usize, height: usize, depth: usize) -> Size {
     sys::MTLSizeMake(width as NSUInteger, height as NSUInteger, depth as NSUInteger)
 }
