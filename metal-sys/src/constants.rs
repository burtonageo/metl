/// Values that indicate specific hardware feature sets that are available.
/// Each GPU family provides a different set of features andhardware limits.
/// For more information, see [Metal Programming Guide](
/// https://developer.apple.com/library/prerelease/ios/documentation/
///     Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/
///     Introduction.html#//apple_ref/doc/uid/TP40014221
/// ).
#[cfg(target_os = "ios")]
#[repr(C, usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLFeatureSet{
    /// The baseline feature set supported by the first generation of iOS GPUs that support Metal.
    /// This feature set is supported by the A7 GPU.
    ///
    /// #Availablility
    ///
    /// Available in iOS 8.0 and later.
    MTLFeatureSet_iOS_GPUFamily1_v1 = 0,
    
    /// The baseline feature set supported by the second generation of iOS GPUs that support Metal.
    /// This feature set is supported by the A8 GPU.
    ///
    /// #Availablility
    ///
    /// Available in iOS 8.0 and later.
    MTLFeatureSet_iOS_GPUFamily2_v1 = 1,
    
    /// The extended feature set supported by the first generation of iOS GPUs that support Metal.
    /// This feature set is supported by the A7 GPU.
    ///
    /// #Availablility
    ///
    /// Available in iOS 9.0 and later.
    MTLFeatureSet_iOS_GPUFamily1_v2 = 2,

    /// The extended feature set supported by the second generation of iOS GPUs that support Metal.
    /// This feature set is supported by the A8 GPU.
    ///
    /// #Availablility
    ///
    /// Available in iOS 9.0 and later.
    MTLFeatureSet_iOS_GPUFamily2_v2 = 3,

    /// The feature set supported by the third generation of iOS GPUs that support Metal.
    /// This feature set is supported by the A9 GPU.
    ///
    /// #Availablility
    ///
    /// Available in iOS 9.0 and later.
    MTLFeatureSet_iOS_GPUFamily3_v1 = 4
}

/// Values that indicate specific hardware feature sets that are available.
/// Each GPU family provides a different set of features andhardware limits.
/// For more information, see [Metal Programming Guide](
/// https://developer.apple.com/library/prerelease/mac/documentation/
///     Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/
///     Introduction.html#//apple_ref/doc/uid/TP40014221
/// ).
#[cfg(target_os = "macos")]
#[repr(C, usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLFeatureSet {
    /// The feature set supported by all OS X GPUs that support Metal.
    MTLFeatureSet_OSX_GPUFamily1_v1 = 10000,

    #[doc(hidden)]
    _non_unary_compile_dummy = -1
}

/// Controls which argument information is made available for reflection by the creation of the pipeline.
#[repr(C, usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLPipelineOption {
   MTLPipelineOptionNone = 0,
   MTLPipelineOptionArgumentInfo = 1 << 0,
   MTLPipelineOptionBufferTypeInfo = 1 << 1
}

#[repr(C, usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLCompareFunction {
    MTLCompareFunctionNever = 0,
    MTLCompareFunctionLess = 1,
    MTLCompareFunctionEqual = 2,
    MTLCompareFunctionLessEqual = 3,
    MTLCompareFunctionGreater = 4,
    MTLCompareFunctionNotEqual = 5,
    MTLCompareFunctionGreaterEqual = 6,
    MTLCompareFunctionAlways = 7
}

#[repr(C, usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLPixelFormat {
    MTLPixelFormatInvalid = 0,
    MTLPixelFormatA8Unorm = 1,
    MTLPixelFormatR8Unorm = 10,
    MTLPixelFormatR8Snorm = 12,
    MTLPixelFormatR8Uint = 13,
    MTLPixelFormatR8Sint = 14,
    MTLPixelFormatR16Unorm = 20,
    MTLPixelFormatR16Snorm = 22,
    MTLPixelFormatR16Uint = 23,
    MTLPixelFormatR16Sint = 24,
    MTLPixelFormatR16Float = 25,
    MTLPixelFormatRG8Unorm = 30,
    MTLPixelFormatRG8Snorm = 32,
    MTLPixelFormatRG8Uint = 33,
    MTLPixelFormatRG8Sint = 34,
    MTLPixelFormatR32Uint = 53,
    MTLPixelFormatR32Sint = 54,
    MTLPixelFormatR32Float = 55,
    MTLPixelFormatRG16Unorm = 60,
    MTLPixelFormatRG16Snorm = 62,
    MTLPixelFormatRG16Uint = 63,
    MTLPixelFormatRG16Sint = 64,
    MTLPixelFormatRG16Float = 65,
    MTLPixelFormatRGBA8Unorm = 70,
    MTLPixelFormatRGBA8Unorm_sRGB = 71,
    MTLPixelFormatRGBA8Snorm = 72,
    MTLPixelFormatRGBA8Uint = 73,
    MTLPixelFormatRGBA8Sint = 74,
    MTLPixelFormatBGRA8Unorm = 80,
    MTLPixelFormatBGRA8Unorm_sRGB = 81,
    MTLPixelFormatRGB10A2Unorm = 90,
    MTLPixelFormatRGB10A2Uint = 91,
    MTLPixelFormatRG11B10Float = 92,
    MTLPixelFormatRGB9E5Float = 93,
    MTLPixelFormatRG32Uint = 103,
    MTLPixelFormatRG32Sint = 104,
    MTLPixelFormatRG32Float = 105,
    MTLPixelFormatRGBA16Unorm = 110,
    MTLPixelFormatRGBA16Snorm = 112,
    MTLPixelFormatRGBA16Uint = 113,
    MTLPixelFormatRGBA16Sint = 114,
    MTLPixelFormatRGBA16Float = 115,
    MTLPixelFormatRGBA32Uint = 123,
    MTLPixelFormatRGBA32Sint = 124,
    MTLPixelFormatRGBA32Float = 125,
    MTLPixelFormatBC1_RGBA = 130,
    MTLPixelFormatBC1_RGBA_sRGB = 131,
    MTLPixelFormatBC2_RGBA = 132,
    MTLPixelFormatBC2_RGBA_sRGB = 133,
    MTLPixelFormatBC3_RGBA = 134,
    MTLPixelFormatBC3_RGBA_sRGB = 135,
    MTLPixelFormatBC4_RUnorm = 140,
    MTLPixelFormatBC4_RSnorm = 141,
    MTLPixelFormatBC5_RGUnorm = 142,
    MTLPixelFormatBC5_RGSnorm = 143,
    MTLPixelFormatBC6H_RGBFloat = 150,
    MTLPixelFormatBC6H_RGBUfloat = 151,
    MTLPixelFormatBC7_RGBAUnorm = 152,
    MTLPixelFormatBC7_RGBAUnorm_sRGB = 153,
    MTLPixelFormatGBGR422 = 240,
    MTLPixelFormatBGRG422 = 241,
    MTLPixelFormatDepth32Float = 252,
    MTLPixelFormatStencil8 = 253,
    MTLPixelFormatDepth24Unorm_Stencil8 = 255,
    MTLPixelFormatDepth32Float_Stencil8 = 260
}
