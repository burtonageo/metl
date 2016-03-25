use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;
use objc::runtime::BOOL;
use {MTLCPUCacheMode, MTLPixelFormat, MTLResourceOptions, MTLStorageMode, MTLTextureType,
     MTLTextureUsage};

/// A `MTLTextureDescriptor` object is used to configure new texture objects.
///
/// A `MTLTexture` object is specific to an individual device. To create a new texture,
/// first create a `MTLTextureDescriptor` object and set its property values, including
/// the texture’s type, size (width, height, and depth), pixel format, number of mipmap
/// levels, sample count (for multisampling), and memory allocation behavior. Then, call
/// either the `newTextureWithDescriptor:` or `newTextureWithDescriptor:iosurface:plane:`
/// method of a `MTLDevice` object, or the `newTextureWithDescriptor:offset:bytesPerRow:`
/// method of a `MTLBuffer` object.
///
/// You can reuse a `MTLTextureDescriptor` object, modifying its property values as needed,
/// to create more `MTLTexture` objects. Texture descriptor properties are used only during
/// the creation of a MTLTexture object. After the texture has been created, property
/// changes in its descriptor have no further effects on it.
///
/// For 2D and cube textures, there are convenience methods to create an MTLTextureDescriptor
/// object: `texture2DDescriptorWithPixelFormat:width:height:mipmapped:` and
/// `textureCubeDescriptorWithPixelFormat:size:mipmapped:`.
pub trait MTLTextureDescriptor {
    /// Creates a texture descriptor object for a 2D texture.
    unsafe fn texture2DDescriptorWithPixelFormat_width_height_mipmapped(
        _: Self, format: MTLPixelFormat, width: NSUInteger, height: NSUInteger,
        mipmapped: BOOL) -> id {
        msg_send![class("MTLTextureDescriptor"), texture2DDescriptorWithPixelFormat:format
                                                                              width:width
                                                                             height:height
                                                                          mipmapped:mipmapped]
    }

    /// Creates a texture descriptor object for a cube texture.
    unsafe fn textureCubeDescriptorWithPixelFormat_size_mipmapped(
        _: Self, format: MTLPixelFormat, size: NSUInteger, mipmapped: BOOL) -> id {
        msg_send![class("MTLTextureDescriptor"), textureCubeDescriptorWithPixelFormat:format
                                                                                 size:size
                                                                            mipmapped:mipmapped]
    }

    /// The dimension and arrangement of texture image data.
    ///
    /// # Discussion
    ///
    /// The default value is `MTLTexture2D`.
    unsafe fn textureType(self) -> MTLTextureType;
    unsafe fn setTextureType(self, textureType: MTLTextureType);

    /// Format that determines how a pixel is written to, stored, and read from the storage
    /// allocation of the texture.
    ///
    /// # Discussion
    ///
    /// The default value is MTLPixelFormatRGBA8Unorm.
    unsafe fn pixelFormat(self) -> MTLPixelFormat;
    unsafe fn setPixelFormat(self, pixelFormat: MTLPixelFormat);

    /// The width of the texture image for the base level mipmap, in pixels.
    ///
    /// # Discussion
    ///
    /// The default value is 1. The value must be greater than or equal to `1`.
    unsafe fn width(self) -> NSUInteger;
    unsafe fn setWidth(self, width: NSUInteger);

    /// The height of the texture image for the base level mipmap, in pixels.
    ///
    /// # Discussion
    ///
    /// The default value is 1. The value must be greater than or equal to `1`.
    /// For a 1D texture, the value must be `1`.
    unsafe fn height(self) -> NSUInteger;
    unsafe fn setHeight(self, height: NSUInteger);

    /// The depth of the texture image for the base level mipmap, in pixels.
    ///
    /// # Discussion
    ///
    /// The default value is `1`. The value must be greater than or equal to `1`.
    /// For 1D, 2D, and cube textures, the value must be `1`.
    unsafe fn depth(self) -> NSUInteger;
    unsafe fn setDepth(self, depth: NSUInteger);

    /// The number of mipmap levels for this texture.
    ///
    /// # Discussion
    ///
    /// The default value is `1`. For a buffer-backed or multisample textures,
    /// the value must be `1`.
    unsafe fn mipmapLevelCount(self) -> NSUInteger;
    unsafe fn setMipmapLevelCount(self, mipmapLevelCount: NSUInteger);

    /// The number of samples in each pixel.
    ///
    /// # Discussion
    ///
    /// The default value is 1. If `textureType` is not `MTLTextureType2DMultisample`,
    /// the value must be 1.
    unsafe fn sampleCount(self) -> NSUInteger;
    unsafe fn setSampleCount(self, sampleCount: NSUInteger);

    /// The number of array elements for a `MTLTextureType1DArray` or `MTLTextureType2DArray`
    /// type texture object.
    ///
    /// # Discussion
    ///
    /// The default value is `1`. The value must be between 1 and 2048, inclusive.
    unsafe fn arrayLength(self) -> NSUInteger;
    unsafe fn setArrayLength(self, arrayLength: NSUInteger);

    /// The behavior of a new memory allocation.
    ///
    /// # Discussion
    ///
    /// This property only has an effect when you are allocating new memory. If you are
    /// creating a texture from buffer data or creating a texture view from another texture,
    /// this property value is ignored.
    unsafe fn resourceOptions(self) -> MTLResourceOptions;
    unsafe fn setResourceOptions(self, resourceOptions: MTLResourceOptions);

    /// The CPU cache mode used for the CPU mapping of the texture.
    ///
    /// # Discussion
    ///
    /// The default value is `MTLCPUCacheModeDefaultCache`.
    unsafe fn cpuCacheMode(self) -> MTLCPUCacheMode;
    unsafe fn setCpuCacheMode(self, cpuCacheMode: MTLCPUCacheMode);

    /// The storage mode used for the location and mapping of the texture.
    ///
    /// # Discsussion
    ///
    /// The default value is `MTLStorageModeAuto`.
    unsafe fn storageMode(self) -> MTLStorageMode;
    unsafe fn setStorageMode(self, storageMode: MTLStorageMode);

    /// Describes how the texture will be used in your app.
    ///
    /// # Discussion
    ///
    /// The default value is `MTLTextureUsageShaderRead` in OS X and `MTLTextureUsageUnknown` in
    /// iOS. You should always aim to determine and set specific texture usages; do not rely on
    /// the `MTLTextureUsageUnknown` value for the best performance.
    unsafe fn usage(self) -> MTLTextureUsage;
    unsafe fn setUsage(self, usage: MTLTextureUsage);
}

impl MTLTextureDescriptor for id {    
    unsafe fn textureType(self) -> MTLTextureType {
        msg_send![self, textureType]
    }

    unsafe fn setTextureType(self, textureType: MTLTextureType) {
        msg_send![self, setTextureType: textureType]
    }

    unsafe fn pixelFormat(self) -> MTLPixelFormat {
        msg_send![self, pixelFormat]
    }

    unsafe fn setPixelFormat(self, pixelFormat: MTLPixelFormat) {
        msg_send![self, setPixelFormat: pixelFormat]
    }

    unsafe fn width(self) -> NSUInteger {
        msg_send![self, width]
    }

    unsafe fn setWidth(self, width: NSUInteger) {
        msg_send![self, setWidth:width]
    }

    unsafe fn height(self) -> NSUInteger {
        msg_send![self, height]
    }

    unsafe fn setHeight(self, height: NSUInteger) {
        msg_send![self, setHeight:height]
    }

    unsafe fn depth(self) -> NSUInteger {
        msg_send![self, depth]
    }

    unsafe fn setDepth(self, depth: NSUInteger) {
        msg_send![self, setDepth:depth]
    }

    unsafe fn mipmapLevelCount(self) -> NSUInteger {
        msg_send![self, mipmapLevelCount]
    }

    unsafe fn setMipmapLevelCount(self, mipmapLevelCount: NSUInteger) {
        msg_send![self, setMipmapLevelCount:mipmapLevelCount]
    }

    unsafe fn sampleCount(self) -> NSUInteger {
        msg_send![self, sampleCount]
    }

    unsafe fn setSampleCount(self, sampleCount: NSUInteger) {
        msg_send![self, setSampleCount:sampleCount]
    }

    unsafe fn arrayLength(self) -> NSUInteger {
        msg_send![self, arrayLength]
    }

    unsafe fn setArrayLength(self, arrayLength: NSUInteger) {
        msg_send![self, setArrayLength:arrayLength]
    }

    unsafe fn resourceOptions(self) -> MTLResourceOptions {
        msg_send![self, resourceOptions]
    }

    unsafe fn setResourceOptions(self, resourceOptions: MTLResourceOptions) {
        msg_send![self, setResourceOptions:resourceOptions]
    }

    unsafe fn cpuCacheMode(self) -> MTLCPUCacheMode {
        msg_send![self, cpuCacheMode]
    }

    unsafe fn setCpuCacheMode(self, cpuCacheMode: MTLCPUCacheMode) {
        msg_send![self, setCpuCacheMode:cpuCacheMode]
    }

    unsafe fn storageMode(self) -> MTLStorageMode {
        msg_send![self, storageMode]
    }

    unsafe fn setStorageMode(self, storageMode: MTLStorageMode) {
        msg_send![self, setStorageMode:storageMode]
    }

    unsafe fn usage(self) -> MTLTextureUsage {
        msg_send![self, usage]
    }

    unsafe fn setUsage(self, usage: MTLTextureUsage) {
        msg_send![self, setUsage:usage]
    }
}
