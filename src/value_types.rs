#![allow(unused_imports)]

use cocoa::foundation::NSUInteger;
use std::convert::{From, Into};
use sys::{MTLClearColor, MTLDispatchThreadgroupsIndirectArguments, MTLDrawIndexedPrimitivesIndirectArguments,
          MTLDrawPrimitivesIndirectArguments, MTLOrigin, MTLRegion, MTLScissorRect, MTLSize, MTLViewport};
use sys::{MTLClearColorMake, MTLOriginMake, MTLSizeMake};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct ClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64
}

impl ClearColor {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        ClearColor {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        }
    }
}

impl From<[f64; 4]> for ClearColor {
    fn from(slice: [f64; 4]) -> Self {
        ClearColor::new(slice[0], slice[1], slice[2], slice[3])
    }
}

impl From<(f64, f64, f64, f64)> for ClearColor {
    fn from((r, g, b, a): (f64, f64, f64, f64)) -> Self {
        ClearColor::new(r, g, b, a)
    }
}

impl From<MTLClearColor> for ClearColor {
    fn from(mtl_color: MTLClearColor) -> Self {
        ClearColor::new(mtl_color.red, mtl_color.green, mtl_color.blue, mtl_color.alpha)
    }
}

impl Into<MTLClearColor> for ClearColor {
    fn into(self) -> MTLClearColor {
        MTLClearColorMake(self.red, self.green, self.blue, self.alpha)
    }
}

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct Origin {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl Origin {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Origin {
            x: x,
            y: y,
            z: z
        }
    }
}

impl From<[usize; 3]> for Origin {
    fn from(slice: [usize; 3]) -> Self {
        Origin::new(slice[0], slice[1], slice[2])
    }
}

impl From<(usize, usize, usize)> for Origin {
    fn from((x, y, z): (usize, usize, usize)) -> Self {
        Origin::new(x, y, z)
    }
}

impl From<MTLOrigin> for Origin {
    fn from(mtl_size: MTLOrigin) -> Self {
        Origin::new(mtl_size.x as usize, mtl_size.y as usize, mtl_size.z as usize)
    }
}

impl Into<MTLOrigin> for Origin {
    fn into(self) -> MTLOrigin {
        MTLOriginMake(self.x as NSUInteger, self.y as NSUInteger, self.z as NSUInteger)
    }
}

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct Region {
    pub size: Size,
    pub origin: Origin
}

impl Region {
    pub fn from_parts(size: Size, origin: Origin) -> Self {
        Region {
            size: size,
            origin: origin
        }
    }

    pub fn new_1d(x: usize, width: usize) -> Self {
        let origin = Origin {x: x, y: 0, z: 0};
        let size = Size {width: width, height: 1, depth: 1};
        Region::from_parts(size, origin)
    }

    pub fn new_2d(x: usize, y: usize, width: usize, height: usize) -> Self {
        let origin = Origin {x: x, y: y, z: 0};
        let size = Size {width: width, height: height, depth: 1};
        Region::from_parts(size, origin)
    }

    pub fn new_3d(x: usize, y: usize, z: usize, width: usize, height: usize, depth: usize) -> Self {
        let origin = Origin {x: x, y: y, z: z};
        let size = Size {width: width, height: height, depth: depth};
        Region::from_parts(size, origin)
    }
}

impl From<MTLRegion> for Region {
    fn from(mtl_region: MTLRegion) -> Self {
        Region::from_parts(mtl_region.size.into(), mtl_region.origin.into())
    }
}

impl Into<MTLRegion> for Region {
    fn into(self) -> MTLRegion {
        MTLRegion {
            size: self.size.into(),
            origin: self.origin.into()
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct ScissorRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize
}

impl From<MTLScissorRect> for ScissorRect {
    fn from(mtl_scissor_rect: MTLScissorRect) -> Self {
        ScissorRect {
            x: mtl_scissor_rect.x as usize,
            y: mtl_scissor_rect.y as usize,
            width: mtl_scissor_rect.width as usize,
            height: mtl_scissor_rect.height as usize
        }
    }
}

impl Into<MTLScissorRect> for ScissorRect {
    fn into(self) -> MTLScissorRect {
        MTLScissorRect {
            x: self.x as NSUInteger,
            y: self.y as NSUInteger,
            width: self.width as NSUInteger,
            height: self.height as NSUInteger
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
    pub depth: usize
}

impl Size {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Size {
            width: width,
            height: height,
            depth: depth
        }
    }
}

impl From<[usize; 3]> for Size {
    fn from(slice: [usize; 3]) -> Self {
        Size::new(slice[0], slice[1], slice[2])
    }
}

impl From<(usize, usize, usize)> for Size {
    fn from((w, h, d): (usize, usize, usize)) -> Self {
        Size::new(w, h, d)
    }
}

impl From<MTLSize> for Size {
    fn from(mtl_size: MTLSize) -> Self {
        Size::new(mtl_size.width as usize, mtl_size.height as usize, mtl_size.depth as usize)
    }
}

impl Into<MTLSize> for Size {
    fn into(self) -> MTLSize {
        MTLSizeMake(self.width as NSUInteger, self.height as NSUInteger, self.depth as NSUInteger)
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Viewport {
    /// The x coordinate of the upper-left corner of the viewport.
    pub origin_x: f64,
    
    /// The y coordinate of the upper-left corner of the viewport.
    pub origin_y: f64,
    
    /// The width of the viewport, in pixels.
    pub width: f64,
    
    /// The height of the viewport, in pixels.
    pub height: f64,
    
    /// The z coordinate of the near clipping plane of the viewport.
    pub znear: f64,
    
    /// The z coordinate of the far clipping plane of the viewport.
    pub zfar: f64
}

impl Viewport {
    pub fn new(ox: f64, oy: f64, w: f64, h: f64, znear: f64, zfar: f64) -> Self {
        Viewport {
            x_origin: ox,
            y_origin: oy,
            width: w,
            height: h,
            znear: znear,
            zfar: zfar
        }
    }
}

impl From<MTLViewport> for Viewport {
    fn from(mtl_viewport: MTLViewport) -> Self {
        Viewport::new(mtl_viewport.originX,
                      mtl_viewport.originY,
                      mtl_viewport.width,
                      mtl_viewport.height,
                      mtl_viewport.znear,
                      mtl_viewport.zfar)
    }
}

impl Into<MTLViewport> for Viewport {
    fn into(self) -> MTLViewport {
        MTLViewport {
            originX: self.x_origin,
            originY: self.y_origin,
            width: self.width,
            height: self.height,
            znear: self.znear,
            zfar: self.zfar
        }
    }
}
