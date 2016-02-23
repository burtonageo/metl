use sys::{MTLCompareFunction, MTLPipelineOption, MTLPixelFormat};

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum PipelineOption: MTLPipelineOption {
        None => MTLPipelineOptionNone,
        ArgumentInfo => MTLPipelineOptionArgumentInfo,
        BufferTypeInfo => MTLPipelineOptionBufferTypeInfo
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum CompareFunction: MTLCompareFunction {
        Never => MTLCompareFunctionNever,
        Less => MTLCompareFunctionLess,
        Equal => MTLCompareFunctionEqual,
        LessEqual => MTLCompareFunctionLessEqual,
        Greater => MTLCompareFunctionGreater,
        NotEqual => MTLCompareFunctionNotEqual,
        GreaterEqual => MTLCompareFunctionGreaterEqual,
        Always => MTLCompareFunctionAlways
    }
}


convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum PixelFormat: MTLPixelFormat {
        Invalid => MTLPixelFormatInvalid,
        A8Unorm => MTLPixelFormatA8Unorm,
        R8Unorm => MTLPixelFormatR8Unorm,
        R8Snorm => MTLPixelFormatR8Snorm,
        R8Uint => MTLPixelFormatR8Uint,
        R8Sint => MTLPixelFormatR8Sint,
        R16Unorm => MTLPixelFormatR16Unorm,
        R16Snorm => MTLPixelFormatR16Snorm,
        R16Uint => MTLPixelFormatR16Uint,
        R16Sint => MTLPixelFormatR16Sint,
        R16Float => MTLPixelFormatR16Float,
        Rg8Unorm => MTLPixelFormatRG8Unorm,
        Rg8Snorm => MTLPixelFormatRG8Snorm,
        Rg8Uint => MTLPixelFormatRG8Uint,
        Rg8Sint => MTLPixelFormatRG8Sint,
        R32Uint => MTLPixelFormatR32Uint,
        R32Sint => MTLPixelFormatR32Sint,
        R32Float => MTLPixelFormatR32Float,
        Rg16Unorm => MTLPixelFormatRG16Unorm,
        Rg16Snorm => MTLPixelFormatRG16Snorm,
        Rg16Uint => MTLPixelFormatRG16Uint,
        Rg16Sint => MTLPixelFormatRG16Sint,
        Rg16Float => MTLPixelFormatRG16Float,
        Rgba8Unorm => MTLPixelFormatRGBA8Unorm,
        Rgba8UnormSrgb => MTLPixelFormatRGBA8Unorm_sRGB,
        Rgba8Snorm => MTLPixelFormatRGBA8Snorm,
        Rgba8Uint => MTLPixelFormatRGBA8Uint,
        Rgba8Sint => MTLPixelFormatRGBA8Sint,
        Bgra8Unorm => MTLPixelFormatBGRA8Unorm,
        Bgra8UnormSrgb => MTLPixelFormatBGRA8Unorm_sRGB,
        Rgb10A2Unorm => MTLPixelFormatRGB10A2Unorm,
        Rgb10A2Uint => MTLPixelFormatRGB10A2Uint,
        Rg11B10Float => MTLPixelFormatRG11B10Float,
        Rgb9E5Float => MTLPixelFormatRGB9E5Float,
        Rg32Uint => MTLPixelFormatRG32Uint,
        Rg32Sint => MTLPixelFormatRG32Sint,
        Rg32Float => MTLPixelFormatRG32Float,
        Rgba16Unorm => MTLPixelFormatRGBA16Unorm,
        Rgba16Snorm => MTLPixelFormatRGBA16Snorm,
        Rgba16Uint => MTLPixelFormatRGBA16Uint,
        Rgba16Sint => MTLPixelFormatRGBA16Sint,
        Rgba16Float => MTLPixelFormatRGBA16Float,
        Rgba32Uint => MTLPixelFormatRGBA32Uint,
        Rgba32Sint => MTLPixelFormatRGBA32Sint,
        Rgba32Float => MTLPixelFormatRGBA32Float,
        Bc1Rgba => MTLPixelFormatBC1_RGBA,
        Bc1RgbaSrgb => MTLPixelFormatBC1_RGBA_sRGB,
        Bc2Rgba => MTLPixelFormatBC2_RGBA,
        Bc2RgbaSrgb => MTLPixelFormatBC2_RGBA_sRGB,
        Bc3Rgba => MTLPixelFormatBC3_RGBA,
        Bc3RgbaSrgb => MTLPixelFormatBC3_RGBA_sRGB,
        Bc4RUnorm => MTLPixelFormatBC4_RUnorm,
        Bc4RSnorm => MTLPixelFormatBC4_RSnorm,
        Bc5RgUnorm => MTLPixelFormatBC5_RGUnorm,
        Bc5RgSnorm => MTLPixelFormatBC5_RGSnorm,
        Bc6HRgbFloat => MTLPixelFormatBC6H_RGBFloat,
        Bc6HRgbUfloat => MTLPixelFormatBC6H_RGBUfloat,
        Bc7RgbaUnorm => MTLPixelFormatBC7_RGBAUnorm,
        Bc7RgbaUnormSrgb => MTLPixelFormatBC7_RGBAUnorm_sRGB,
        Gbgr422 => MTLPixelFormatGBGR422,
        Bgrg422 => MTLPixelFormatBGRG422,
        Depth32Float => MTLPixelFormatDepth32Float,
        Stencil8 => MTLPixelFormatStencil8,
        Depth24UnormStencil8 => MTLPixelFormatDepth24Unorm_Stencil8,
        Depth32FloatStencil8 => MTLPixelFormatDepth32Float_Stencil8
    }
}
