use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;
use core_foundation::base::CFRange;
use libc::c_void;
use objc::runtime::BOOL;
use {MTLPixelFormat, MTLRegion};

#[cfg(target_os = "macos")]
macro_rules! texture_platform_methods_impl {
    () => {
        unsafe fn ioSurface(self) -> id {
            msg_send![self, ioSurface]
        }
        unsafe fn ioSurfacePlane(self) -> NSUInteger {
            msg_send![self, ioSurfacePlane]
        }
    }
}

#[cfg(target_os = "ios")]
macro_rules! texture_platform_methods_impl {
    () => {
        unsafe fn buffer(self) -> id {
            msg_send![self, buffer]
        }

        unsafe fn bufferOffset(self) -> NSUInteger {
            msg_send![self, bufferOffset]
        }

        unsafe fn bufferBytesPerRow(self) -> id {
            msg_send![self, bufferBytesPerRow]
        }
    }
}

#[cfg(target_os = "macos")]
pub trait MTLTexture {
    unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
        self, region: MTLRegion, level: NSUInteger, slice: NSUInteger,
        pixelBytes: *const c_void, bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger);

    unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(self, region: MTLRegion,
                                                              level: NSUInteger,
                                                              pixelBytes: *const c_void,
                                                              bytesPerRow: NSUInteger);

    unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
        self, pixel_bytes: *mut c_void, bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger, region: MTLRegion, mipmapLevel: NSUInteger,
        slice: NSUInteger);

    unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(self, pixel_bytes: *mut c_void,
                                                          bytesPerRow: NSUInteger,
                                                          region: MTLRegion,
                                                          mipmapLevel: NSUInteger);

    unsafe fn newTextureViewWithPixelFormat(_: Self, format: MTLPixelFormat) -> id {
        msg_send![class("MTLTexture"), newTextureViewWithPixelFormat:format]
    }

    unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices(
        _: Self, format: MTLPixelFormat, textureType: MTLTextureType,
        levels: CFRange, slices: NSUInteger) -> id {
        msg_send![class("MTLTexture"), newTextureViewWithPixelFormat:format
                                                         textureType:textureType
                                                              levels:levels
                                                              slices:slices]
    }

    unsafe fn textureType(self) -> MTLTextureType;
    unsafe fn pixelFormat(self) -> MTLPixelFormat;
    unsafe fn width(self) -> NSUInteger;
    unsafe fn height(self) -> NSUInteger;
    unsafe fn depth(self) -> NSUInteger;
    unsafe fn mipmapLevelCount(self) -> NSUInteger;
    unsafe fn arrayLength(self) -> NSUInteger;
    unsafe fn sampleCount(self) -> NSUInteger;
    unsafe fn frameBufferOnly(self) -> BOOL;
    unsafe fn rootResource(self) -> id;
    unsafe fn usage(self) -> MTLTextureUsage;

    unsafe fn ioSurface(self) -> id;
    unsafe fn ioSurfacePlane(self) -> NSUInteger;

    unsafe fn parentTexture(self) -> id;
    unsafe fn parentRelativeLevel(self) -> NSUInteger;
    unsafe fn parentRelativeSlice(self) -> NSUInteger;
}

#[cfg(target_os = "ios")]
pub trait MTLTexture {
    unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
        self, region: MTLRegion, level: NSUInteger, slice: NSUInteger,
        pixelBytes: *const c_void, bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger);

    unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(self, region: MTLRegion,
                                                              level: NSUInteger,
                                                              pixelBytes: *mut c_void,
                                                              bytesPerRow: NSUInteger);

    unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
        self, pixel_bytes: *mut c_void, bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger, region: MTLRegion, mipmapLevel: NSUInteger,
        slice: NSUInteger);

    unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(self, pixel_bytes: *mut c_void,
                                                          bytesPerRow: NSUInteger,
                                                          region: MTLRegion,
                                                          mipmapLevel: NSUInteger);

    unsafe fn newTextureViewWithPixelFormat(format: MTLPixelFormat) -> id {
        msg_send![class("MTLTexture"), newTextureViewWithPixelFormat:format]
    }

    unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices(format: MTLPixelFormat,
                                                                      textureType: MTLTextureType,
                                                                      levels: CFRange,
                                                                      slices: NSUInteger)
                                                                      -> id {
        msg_send![class("MTLTexture"), newTextureViewWithPixelFormat:format
                                                         textureType:textureType
                                                              levels:levels
                                                              slices:slices]
    }

    unsafe fn textureType(self) -> MTLTextureType;
    unsafe fn pixelFormat(self) -> MTLPixelFormat;
    unsafe fn width(self) -> NSUInteger;
    unsafe fn height(self) -> NSUInteger;
    unsafe fn depth(self) -> NSUInteger;
    unsafe fn mipmapLevelCount(self) -> NSUInteger;
    unsafe fn arrayLength(self) -> NSUInteger;
    unsafe fn sampleCount(self) -> NSUInteger;
    unsafe fn frameBufferOnly(self) -> BOOL;
    unsafe fn rootResource(self) -> id;
    unsafe fn usage(self) -> MTLTextureUsage;

    unsafe fn buffer(self) -> id;
    unsafe fn bufferOffset(self) -> NSUInteger;
    unsafe fn bufferBytesPerRow(self) -> NSUInteger;

    unsafe fn parentTexture(self) -> id;
    unsafe fn parentRelativeLevel(self) -> NSUInteger;
    unsafe fn parentRelativeSlice(self) -> NSUInteger;
}


impl MTLTexture for id {
    unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
        self, region: MTLRegion, level: NSUInteger, slice: NSUInteger,
        pixelBytes: *const c_void, bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger) {
        msg_send![self, replaceRegion:region
                          mipmapLevel:level
                                slice:slice
                            withBytes:pixelBytes
                          bytesPerRow:bytesPerRow
                        bytesPerImage:bytesPerImage]
    }
    unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(self, region: MTLRegion,
                                                              level: NSUInteger,
                                                              pixelBytes: *const c_void,
                                                              bytesPerRow: NSUInteger) {
        msg_send![self, replaceRegion:region
                          mipmapLevel:level
                            withBytes:pixelBytes
                          bytesPerRow:bytesPerRow]
    }

    unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
        self, pixelBytes: *mut c_void, bytesPerRow: NSUInteger,
        bytesPerImage: NSUInteger, region: MTLRegion, level: NSUInteger,
        slice: NSUInteger) {
        msg_send![self, getBytes:pixelBytes
                     bytesPerRow:bytesPerRow
                   bytesPerImage:bytesPerImage
                          region:region
                     mipmapLevel:level
                           slice:slice]
    }

    unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(self, pixelBytes: *mut c_void,
                                                          bytesPerRow: NSUInteger,
                                                          region: MTLRegion, level: NSUInteger) {
        msg_send![self, getBytes:pixelBytes
                     bytesPerRow:bytesPerRow
                          region:region
                     mipmapLevel:level]
    }

    unsafe fn textureType(self) -> MTLTextureType {
        msg_send![self, textureType]
    }

    unsafe fn pixelFormat(self) -> MTLPixelFormat {
        msg_send![self, pixelFormat]
    }

    unsafe fn width(self) -> NSUInteger {
        msg_send![self, width]
    }

    unsafe fn height(self) -> NSUInteger {
        msg_send![self, height]
    }

    unsafe fn depth(self) -> NSUInteger {
        msg_send![self, depth]
    }

    unsafe fn mipmapLevelCount(self) -> NSUInteger {
        msg_send![self, mipmapLevelCount]
    }

    unsafe fn arrayLength(self) -> NSUInteger {
        msg_send![self, arrayLength]
    }

    unsafe fn sampleCount(self) -> NSUInteger {
        msg_send![self, sampleCount]
    }

    unsafe fn frameBufferOnly(self) -> BOOL {
        msg_send![self, frameBufferOnly]
    }

    unsafe fn rootResource(self) -> id {
        msg_send![self, rootResource]
    }

    unsafe fn usage(self) -> MTLTextureUsage {
        msg_send![self, usage]
    }

    unsafe fn parentTexture(self) -> id {
        msg_send![self, parentTexture]
    }

    unsafe fn parentRelativeLevel(self) -> NSUInteger {
        msg_send![self, parentRelativeLevel]
    }

    unsafe fn parentRelativeSlice(self) -> NSUInteger {
        msg_send![self, parentRelativeSlice]
    }

    texture_platform_methods_impl!{}
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLTextureType {
    MTLTextureType1D = 0,
    MTLTextureType1DArray = 1,
    MTLTextureType2D = 2,
    MTLTextureType2DArray = 3,
    MTLTextureType2DMultisample = 4,
    MTLTextureTypeCube = 5,
    MTLTextureType3D = 7
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLTextureUsage {
    MTLTextureUsageUnknown = 0x0000,
    MTLTextureUsageShaderRead = 0x0001,
    MTLTextureUsageShaderWrite = 0x0002,
    MTLTextureUsageRenderTarget = 0x0004,
    MTLTextureUsagePixelFormatView = 0x0010
}
