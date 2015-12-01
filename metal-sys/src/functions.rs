use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use libc::c_void;
use {MTLClearColor, MTLSize, MTLOrigin, MTLRegion};

#[link(name = "Metal", kind = "framework")]
extern {
    pub fn MTLCopyAllDevices() -> id;
    pub fn MTLCreateSystemDefaultDevice() -> id;
}

pub fn MTLOriginMake(x: NSUInteger, y: NSUInteger, z: NSUInteger) -> MTLOrigin {
    MTLOrigin {
        x: x,
        y: y,
        z: z
    }
}

pub fn MTLRegionMake1D(x: NSUInteger, width: NSUInteger) -> MTLRegion {
    MTLRegion {
        origin: MTLOriginMake(x, 0, 0),
        size: MTLSizeMake(width, 1, 1)
    }
}

pub fn MTLRegionMake2D(x: NSUInteger, y: NSUInteger, width: NSUInteger, height: NSUInteger) -> MTLRegion {
    MTLRegion {
        origin: MTLOriginMake(x, y, 0),
        size: MTLSizeMake(width, height, 1)
    }
}

pub fn MTLRegionMake3D(x: NSUInteger,
                       y: NSUInteger,
                       z: NSUInteger,
                       width: NSUInteger,
                       height: NSUInteger,
                       depth: NSUInteger) -> MTLRegion {
    MTLRegion {
        origin: MTLOriginMake(x, y, z),
        size: MTLSizeMake(width, height, depth)
    }
}

pub fn MTLSizeMake (width: NSUInteger, height: NSUInteger, depth: NSUInteger) -> MTLSize {
    MTLSize {
        width: width,
        height: height,
        depth: depth
    }
}

/// Returns a value used to clear a color attachment (in effect, when the loadAction property of
/// MTLRenderPassAttachmentDescriptor is MTLLoadActionClear).
///
/// # Arguments
///
/// * `red` - The red color channel clearing value.
/// * `green` - The green color channel clearing value.
/// * `blue` - The blue color channel clearing value.
/// * `alpha` - The alpha color channel clearing value.
///
/// # Return Value
///
/// A value for clearing a color attachment.
pub fn MTLClearColorMake(red: f64, green: f64, blue: f64, alpha: f64) -> MTLClearColor {
    MTLClearColor {
        red: red,
        green: green,
        blue: blue,
        alpha: alpha
    }
}
