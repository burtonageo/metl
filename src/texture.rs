use cocoa::base::{id, nil};
use cocoa::foundation::{NSInteger, NSUInteger};
use core_foundation::base::CFRange;
use objc::runtime::YES;
use std::ops::Range;
use sys::{MTLTexture, MTLTextureType, MTLTextureUsage};
use {FromRaw, FromRawError, PixelFormat, Region, Resource, Size};
#[cfg(target_os = "ios")]
use Buffer;

pub struct Texture(id);

impl Texture {
    pub fn with_pixel_format(format: PixelFormat) -> Result<Self, FromRawError> {
        let raw_tex = unsafe { MTLTexture::newTextureViewWithPixelFormat(nil, format.into()) };
        Self::from_raw(raw_tex)
    }

    pub fn new(format: PixelFormat, tex_type: TextureType, levels: Range<isize>, slices: usize)
               -> Result<Self, FromRawError> {
        let cf_levels = CFRange::init(levels.start as NSInteger,
                                      (levels.end - levels.start) as NSInteger);
        let raw_tex = unsafe {
            MTLTexture::newTextureViewWithPixelFormat_textureType_levels_slices(
                nil, format.into(), tex_type.into(), cf_levels,
                slices as NSUInteger)
        };
        Self::from_raw(raw_tex)
    }

    pub fn replace_region_at_slice(&mut self, region: Region, slice: usize, mipmap_level: usize,
                                   bytes: &AsRef<[u8]>, bytes_per_row: usize,
                                   bytes_per_image: usize) {
        unsafe {
            self.0.replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
                region.into(), mipmap_level as NSUInteger, slice as NSUInteger,
                bytes.as_ref().as_ptr() as *const _, bytes_per_row as NSUInteger,
                bytes_per_image as NSUInteger);
        }
    }

    pub fn replace_region(&mut self, mipmap_level: usize, region: Region, bytes: &AsRef<[u8]>,
                          bytes_per_row: usize) {
        let bytes = bytes.as_ref();
        if bytes.len() % bytes_per_row != 0 {
            return;
        }
        unsafe {
            self.0.replaceRegion_mipmapLevel_withBytes_bytesPerRow(region.into(),
                                                                   mipmap_level as NSUInteger,
                                                                   bytes.as_ptr() as *const _,
                                                                   bytes_per_row as NSUInteger);
        }
    }

    // TODO(George): Investigate non-allocating APIs, and add predicates in rust code so that
    // panics can be generated
    pub fn get_bytes_at_slice(&self, bytes_storage: &mut Vec<u8>, bytes_per_row: usize,
                              bytes_per_image: usize, region: Region, mipmap_level: usize,
                              slice: usize) {
        bytes_storage.resize(self.width() * self.height() * self.depth(), 0u8);
        unsafe {
            self.0.getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
                bytes_storage.as_mut_ptr() as *mut _, bytes_per_row as NSUInteger,
                bytes_per_image as NSUInteger, region.into(), mipmap_level as NSUInteger,
                slice as NSUInteger);
        }
        bytes_storage.shrink_to_fit();
    }

    pub fn get_bytes(&self, bytes_storage: &mut Vec<u8>, bytes_per_row: usize, region: Region,
                     mipmap_level: usize) {
        bytes_storage.resize(self.width() * self.height() * self.depth(), 0u8);
        unsafe {
            self.0
                .getBytes_bytesPerRow_fromRegion_mipmapLevel(bytes_storage.as_mut_ptr() as *mut _,
                                                             bytes_per_row as NSUInteger,
                                                             region.into(),
                                                             mipmap_level as NSUInteger);
        }
        bytes_storage.shrink_to_fit();
    }

    pub fn texture_type(&self) -> TextureType {
        unsafe { self.0.textureType().into() }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { self.0.pixelFormat().into() }
    }

    pub fn width(&self) -> usize {
        unsafe { self.0.width() as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { self.0.height() as usize }
    }

    pub fn depth(&self) -> usize {
        unsafe { self.0.depth() as usize }
    }

    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height(), self.depth())
    }

    pub fn mipmap_level_count(&self) -> usize {
        unsafe { self.0.mipmapLevelCount() as usize }
    }

    pub fn array_length(&self) -> usize {
        unsafe { self.0.arrayLength() as usize }
    }

    pub fn sample_count(&self) -> usize {
        unsafe { self.0.sampleCount() as usize }
    }

    pub fn is_frame_buffer_only(&self) -> bool {
        unsafe { self.0.frameBufferOnly() == YES }
    }

    pub fn root_resource(&self) -> Option<Resource> {
        unsafe { FromRaw::from_raw(self.0.rootResource()).ok() }
    }

    pub fn usage(&self) -> TextureUsage {
        unsafe { self.0.usage().into() }
    }

    pub fn parent_texture(&self) -> Option<Self> {
        unsafe { Self::from_raw(self.0.parentTexture()).ok() }
    }

    pub fn parent_relative_level(&self) -> usize {
        unsafe { self.0.parentRelativeLevel() as usize }
    }

    pub fn parent_relative_slice(&self) -> usize {
        unsafe { self.0.parentRelativeSlice() as usize }
    }
}

#[cfg(target_os = "macos")]
impl Texture {}

#[cfg(target_os = "ios")]
impl Texture {
    pub fn buffer(&self) -> Option<Buffer> {
        unsafe { FromRaw::from_raw(MTLTexture::buffer(self.0)).ok() }
    }

    pub fn buffer_offset(&self) -> usize {
        unsafe { self.0.bufferOffset() as usize }
    }

    pub fn buffer_bytes_per_row(&self) -> usize {
        unsafe { self.0.bufferBytesPerRow() as usize }
    }
}

impl_from_into_raw!(Texture, of protocol "MTLTexture");

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum TextureType: MTLTextureType {
        Type1D => MTLTextureType1D,
        Type1DArray => MTLTextureType1DArray,
        Type2D => MTLTextureType2D,
        Type2DArray => MTLTextureType2DArray,
        Type2DMultisample => MTLTextureType2DMultisample,
        TypeCube => MTLTextureTypeCube,
        Type3D => MTLTextureType3D
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum TextureUsage: MTLTextureUsage {
        Unknown => MTLTextureUsageUnknown,
        ShaderRead => MTLTextureUsageShaderRead,
        ShaderWrite => MTLTextureUsageShaderWrite,
        RenderTarget => MTLTextureUsageRenderTarget,
        PixelFormatView => MTLTextureUsagePixelFormatView
    }
}
