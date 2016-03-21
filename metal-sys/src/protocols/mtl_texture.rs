use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;
use core_foundation::base::CFRange;
use libc::c_void;
use objc::runtime::BOOL;
use {MTLPixelFormat, MTLRegion};

/// The `MTLTexture` protocol defines the interface for an object that
/// represents an allocation of formatted image data. Textures are used
/// as source data for a vertex shader, a fragment shader, or a compute
/// kernel. They are also used as an attachment that acts as a rendering
/// destination.
///
/// Do not use standard allocation and initialization techniques to create
/// a `MTLTexture` object. The following methods create and return a
/// `MTLTexture` object:
///
/// * The `newTextureWithDescriptor:` method of the MTLDevice protocol creates
///   a texture object with a new storage allocation for the texture image data,
///   using a `MTLTextureDescriptor` object to describe the texture’s properties.
///   To further specify an `IOSurface` from which to create the texture, use the
///   znewTextureWithDescriptor:iosurface:plane:` method instead.
///
///
/// * The `newTextureViewWithPixelFormat:` and
///   `newTextureViewWithPixelFormat:textureType:levels:slices:` methods of the
///   `MTLTexture` protocol create and return a new texture object that shares the
///   same storage allocation as the source texture object. Because they share the
///   same storage, any changes to the pixels of the new texture are reflected in
///   the source texture, and vice versa. For the newly created texture, these
///   methods reinterpret the existing texture image data in the storage allocation
///   of the source texture as if this data were stored in the new specified pixel
///   format. The pixel format of the new texture must be compatible with the pixel
///   format of the source texture.
///
/// * In iOS, the `newTextureWithDescriptor:offset:bytesPerRow:` method of the
///   `MTLBuffer` protocol creates and returns a new texture object that shares the
///   storage allocation of the source buffer object as its texture image data. Because
///   they share the same storage, any changes to the pixels of the new texture are
///   reflected in the source buffer, and vice versa.
///
/// * The `textureType` property identifies how the image data is organized. Image data
///   is arranged in one or more slices, and each slice is a single `MTLTextureType1D`,
///   `MTLTextureType2D`, `MTLTextureType2DMultisample`, or `MTLTextureType3D` texture
///   type image and all its mipmaps. `MTLTextureType1D`, `MTLTextureType2D`,
///   `MTLTextureType2DMultisample`, and `MTLTextureType3D` texture types have a single
///   slice. A `MTLTextureTypeCube` texture always has six slices, one for each face.
///   For a texture array object, such as `MTLTextureType1DArray`, `MTLTextureType2DArray`,
///   or `MTLTextureTypeCubeArray`, every array element is one slice.
///
/// After you create a texture, you can call
/// `replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:` or
/// `replaceRegion:mipmapLevel:withBytes:bytesPerRow:` to populate the storage allocation
/// of the texture object with image data from system memory. Call
/// `getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:` or
/// `getBytes:bytesPerRow:fromRegion:mipmapLevel:` to copy image data from a texture object
/// and store the copied data into system memory.
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

    unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices(_: Self,
                                                                      format: MTLPixelFormat,
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

    #[cfg(target_os = "ios")]
    unsafe fn buffer(self) -> id;
    #[cfg(target_os = "ios")]
    unsafe fn bufferOffset(self) -> NSUInteger;
    #[cfg(target_os = "ios")]
    unsafe fn bufferBytesPerRow(self) -> NSUInteger;

    #[cfg(target_os = "macos")]
    unsafe fn ioSurface(self) -> id;
    #[cfg(target_os = "macos")]
    unsafe fn ioSurfacePlane(self) -> NSUInteger;

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

    #[cfg(target_os = "ios")]
    unsafe fn buffer(self) -> id {
        msg_send![self, buffer]
    }

    #[cfg(target_os = "ios")]
    unsafe fn bufferOffset(self) -> NSUInteger {
        msg_send![self, bufferOffset]
    }

    #[cfg(target_os = "ios")]
    unsafe fn bufferBytesPerRow(self) -> id {
        msg_send![self, bufferBytesPerRow]
    }

    #[cfg(target_os = "macos")]
    unsafe fn ioSurface(self) -> id {
        msg_send![self, ioSurface]
    }

    #[cfg(target_os = "macos")]
    unsafe fn ioSurfacePlane(self) -> NSUInteger {
        msg_send![self, ioSurfacePlane]
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
