use cocoa::base::{class, id};
use cocoa::foundation::NSUInteger;
use objc::runtime::BOOL;
use MTLCompareFunction;

pub trait MTLSamplerDescriptor {
    unsafe fn new(_: Self) -> id {
        msg_send![class("MTLSamplerDescriptor"), new]
    }

    /// The address mode for the texture depth (r) coordinate.
    ///
    /// # Discussion
    ///
    /// The default value is `MTLSamplerAddressModeClampToEdge`.
    unsafe fn rAddressMode(self) -> MTLSamplerAddressMode;
    unsafe fn setRAddressMode(self, addressMode: MTLSamplerAddressMode);

    /// The address mode for the texture width (s) coordinate.
    ///
    /// # Discussion
    ///
    /// The default value is MTLSamplerAddressModeClampToEdge.
    unsafe fn sAddressMode(self) -> MTLSamplerAddressMode;
    unsafe fn setSAddressMode(self, addressMode: MTLSamplerAddressMode);

    /// The address mode for the texture height (t) coordinate.
    ///
    /// # Discsussion
    ///
    /// The default value is `MTLSamplerAddressModeClampToEdge`.
    unsafe fn tAddressMode(self) -> MTLSamplerAddressMode;
    unsafe fn setTAddressMode(self, addressMode: MTLSamplerAddressMode);

    /// The filtering option for combining pixels within one mipmap level when
    /// the sample footprint is larger than a pixel (minification).
    ///
    /// # Discussion
    ///
    /// The default value is `MTLSamplerMinMagFilterNearest`.
    unsafe fn minFilter(self) -> MTLSamplerMinMagFilter;
    unsafe fn setMinFilter(self, minFilter: MTLSamplerMinMagFilter);

    /// The filtering operation for combining pixels within one mipmap level when
    /// the sample footprint is smaller than a pixel (magnification).
    ///
    /// # Discussion
    ///
    /// The default value is `MTLSamplerMinMagFilterNearest`.
    unsafe fn magFilter(self) -> MTLSamplerMinMagFilter;
    unsafe fn setMagFilter(self, magFilter: MTLSamplerMinMagFilter);

    /// The filtering option for combining pixels between two mipmap levels.
    ///
    /// # Discussion
    ///
    /// The default value is `MTLSamplerMipFilterNotMipmapped`.
    unsafe fn mipFilter(self) -> MTLSamplerMipFilter;
    unsafe fn setMipFilter(self, mipFilter: MTLSamplerMipFilter);

    /// The minimum level of detail (LOD) to use when sampling from a texture.
    ///
    /// # Discussion
    ///
    /// The default value is `0.0f`. Clamp values are ignored for texture sample
    /// variants that specify an explicit LOD
    unsafe fn lodMinClamp(self) -> f32;
    unsafe fn setLodMinClamp(self, lodMinClamp: f32);

    /// The maximum level of detail (LOD) to use when sampling from a texture.
    ///
    /// # Discussion
    ///
    /// The default value is `FLT_MAX`. Clamp values are ignored for texture sample
    /// variants that specify an explicit LOD.
    unsafe fn lodMaxClamp(self) -> f32;
    unsafe fn setLodMaxClamp(self, lodMaxClamp: f32);

    /// Allows the driver to use an average level of detail (LOD) when sampling from a texture.
    ///
    /// # Discussion
    ///
    /// If this value is YES, an average LOD may be used across four fragment shader threads.
    /// If this value is NO, no averaging is performed and each thread will access its own LOD.
    unsafe fn lodAverage(self) -> BOOL;
    unsafe fn setLodAverage(self, lodAverage: BOOL);

    /// The number of samples that can be taken to improve the quality of sample footprints that
    /// are anisotropic.
    ///
    /// # Discussion
    ///
    /// Values must be between `1` and `16`, inclusive. The default value is `1`.
    unsafe fn maxAnisotropy(self) -> NSUInteger;
    unsafe fn setMaxAnisotropy(self, maxAnisotropy: NSUInteger);

    /// A Boolean value that indicates whether texture coordinates are normalized to the
    /// range `[0.0, 1.0]`.
    ///
    /// # Discussion
    ///
    /// If `YES`, texture coordinates are from `0.0` to `1.0`. If `NO`, texture coordinates
    /// are from `0` to `width` for horizontal coordinates and `0` to `height` for vertical
    /// coordinates. The default value is `YES`.
    ///
    /// Non-normalized texture coordinates should only be used with 1D and 2D textures with
    /// the following conditions; otherwise, the results of sampling are undefined.
    ///
    /// * The `MTLSamplerAddressModeClampToEdge` or `MTLSamplerAddressModeClampToZero` address
    ///   mode.
    ///
    /// * The `MTLSamplerMipFilterNotMipmapped` mipmap filtering option.
    ///
    /// * `minFilter` and `magFilter` must be equal to each other.
    ///
    /// * `maxAnisotropy` must be `1`.
    unsafe fn normalizedCoordinates(self) -> BOOL;
    unsafe fn setNormalizedCoordinates(self, normalizedCoordinates: BOOL);

    /// The sampler comparison function used when sampling texels from a depth texture.
    ///
    /// # Discussion
    ///
    /// The default value is `MTLCompareFunctionNever`.
    ///
    /// The `MTLFeatureSet_iOS_GPUFamily3_v1` and `MTLFeatureSet_OSX_GPUFamily1_v1` feature
    /// sets allow you to define a framework-side sampler comparison function for a
    /// `MTLSamplerState` object. All feature sets support shader-side sampler comparison
    /// functions, as described in the Metal Shading Language Guide.
    unsafe fn compareFunction(self) -> MTLCompareFunction;
    unsafe fn setCompareFunction(self, compareFunction: MTLCompareFunction);

    /// A string used to identify this sampler descriptor.
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    unsafe fn copy(self) -> id;
}

impl MTLSamplerDescriptor for id {
    unsafe fn rAddressMode(self) -> MTLSamplerAddressMode {
        msg_send![self, rAddressMode]
    }

    unsafe fn setRAddressMode(self, addressMode: MTLSamplerAddressMode) {
        msg_send![self, setRAddressMode:addressMode]
    }

    unsafe fn sAddressMode(self) -> MTLSamplerAddressMode {
        msg_send![self, sAddressMode]
    }

    unsafe fn setSAddressMode(self, addressMode: MTLSamplerAddressMode) {
        msg_send![self, setSAddressMode:addressMode]
    }

    unsafe fn tAddressMode(self) -> MTLSamplerAddressMode {
        msg_send![self, tAddressMode]
    }

    unsafe fn setTAddressMode(self, addressMode: MTLSamplerAddressMode) {
        msg_send![self, setTAddressMode:addressMode]
    }

    unsafe fn minFilter(self) -> MTLSamplerMinMagFilter {
        msg_send![self, minFilter]
    }

    unsafe fn setMinFilter(self, minFilter: MTLSamplerMinMagFilter) {
        msg_send![self, setMinFilter:minFilter]
    }

    unsafe fn magFilter(self) -> MTLSamplerMinMagFilter {
        msg_send![self, magFilter]
    }

    unsafe fn setMagFilter(self, magFilter: MTLSamplerMinMagFilter) {
        msg_send![self, setMagFilter:magFilter]
    }
    
    unsafe fn mipFilter(self) -> MTLSamplerMipFilter {
        msg_send![self, mipFilter]
    }

    unsafe fn setMipFilter(self, mipFilter: MTLSamplerMipFilter) {
        msg_send![self, setMipFilter:mipFilter]
    }

    
    unsafe fn lodMinClamp(self) -> f32 {
        msg_send![self, lodMinClamp]
    }

    unsafe fn setLodMinClamp(self, lodMinClamp: f32) {
        msg_send![self, setLodMinClamp:lodMinClamp]
    }

    unsafe fn lodMaxClamp(self) -> f32 {
        msg_send![self, lodMaxClamp]
    }

    unsafe fn setLodMaxClamp(self, lodMaxClamp: f32) {
        msg_send![self, setLodMaxClamp:lodMaxClamp]
    }

    unsafe fn lodAverage(self) -> BOOL {
        msg_send![self, lodAverage]
    }

    unsafe fn setLodAverage(self, lodAverage: BOOL) {
        msg_send![self, setLodAverage:lodAverage]
    }

    unsafe fn maxAnisotropy(self) -> NSUInteger {
        msg_send![self, maxAnisotropy]
    }

    unsafe fn setMaxAnisotropy(self, maxAnisotropy: NSUInteger) {
        msg_send![self, setMaxAnisotropy:maxAnisotropy]
    }

    unsafe fn normalizedCoordinates(self) -> BOOL {
        msg_send![self, normalizedCoordinates]
    }

    unsafe fn setNormalizedCoordinates(self, normalizedCoordinates: BOOL) {
        msg_send![self, setNormalizedCoordinates:normalizedCoordinates]
    }

    unsafe fn compareFunction(self) -> MTLCompareFunction {
        msg_send![self, compareFunction]
    }

    unsafe fn setCompareFunction(self, compareFunction: MTLCompareFunction) {
        msg_send![self, setCompareFunction:compareFunction]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }

    unsafe fn copy(self) -> id {
        msg_send![self, copy]
    }
}

/// Modes that determine the texture coordinate at each pixel when a fetch
/// falls outside the bounds of a texture. Also known as wrapping mode.
#[repr(usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLSamplerAddressMode {
    /// Texture coordinates are clamped between 0.0 and 1.0, inclusive.
    MTLSamplerAddressModeClampToEdge = 0,

    ///  Texture coordinates wrap to the other side of the texture, effectively
    /// keeping only the fractional part of the texture coordinate.
    MTLSamplerAddressModeRepeat = 2,

    /// Between -1.0 and 1.0, the texture coordinates are mirrored across the axis.
    /// Outside -1.0 and 1.0, the image is repeated.
    MTLSamplerAddressModeMirrorRepeat = 3,

    /// Out of range texture coordinates return transparent zero (0,0,0,0) for images
    /// with an alpha channel and return opaque zero (0,0,0,1) for images without an
    /// alpha channel.
    MTLSamplerAddressModeClampToZero = 4
}

/// Filtering options for determining which pixel value is returned within a mipmap level.
#[repr(usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLSamplerMinMagFilter {
    /// Select the single pixel nearest to the sample point.
    MTLSamplerMinMagFilterNearest = 0,

    /// Select two pixels in each dimension and interpolate linearly between them. Not all
    /// devices support linear filtering for all formats. Integer textures can not use
    /// linear filtering on any device, and only some devices support linear filtering for
    ///Float textures.
    MTLSamplerMinMagFilterLinear = 1
}

/// Filtering options for determining what pixel value is returned with multiple mipmap
/// levels.
#[repr(usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLSamplerMipFilter {
    /// The texture is sampled from mipmap level `0`. Other mipmap levels are ignored.
    MTLSamplerMipFilterNotMipmapped = 0,
    /// The nearest mipmap level is selected.
    MTLSamplerMipFilterNearest = 1,
    /// If the filter falls between mipmap levels, both levels are sampled and the results
    /// are determined by linear interpolation between levels.
    MTLSamplerMipFilterLinear = 2
}
