use cocoa::base::{id, nil};
use cocoa::foundation::{NSInteger, NSUInteger};
use core_foundation::base::CFRange;
use objc::runtime::{BOOL, YES};
use std::mem;
use std::ops::{Deref, Range};
use sys::{MTLTexture, MTLTextureDescriptor, MTLTextureType, MTLTextureUsage};
use {CpuCacheMode, FromRaw, FromRawError, PixelFormat, Region, Resource, ResourceOptions, Size,
     StorageMode};
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
        unsafe { MTLTexture::textureType(self.0).into() }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { MTLTexture::pixelFormat(self.0).into() }
    }

    pub fn width(&self) -> usize {
        unsafe { MTLTexture::width(self.0) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { MTLTexture::height(self.0) as usize }
    }

    pub fn depth(&self) -> usize {
        unsafe { MTLTexture::depth(self.0) as usize }
    }

    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height(), self.depth())
    }

    pub fn mipmap_level_count(&self) -> usize {
        unsafe { MTLTexture::mipmapLevelCount(self.0) as usize }
    }

    pub fn array_length(&self) -> usize {
        unsafe { MTLTexture::arrayLength(self.0) as usize }
    }

    pub fn sample_count(&self) -> usize {
        unsafe { MTLTexture::sampleCount(self.0) as usize }
    }

    pub fn is_frame_buffer_only(&self) -> bool {
        unsafe { self.0.frameBufferOnly() == YES }
    }

    pub fn root_resource(&self) -> Option<Resource> {
        unsafe { FromRaw::from_raw(self.0.rootResource()).ok() }
    }

    pub fn usage(&self) -> TextureUsage {
        unsafe { MTLTexture::usage(self.0).into() }
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

impl Deref for Texture {
    type Target = Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl_from_into_raw!(Texture, of protocol "MTLTexture");

pub struct TextureDescriptor(id);

impl TextureDescriptor {
    pub fn new() -> Self {
        unsafe { FromRaw::from_raw(MTLTextureDescriptor::new(nil)).unwrap() }
    }

    pub fn new_2d(pixel_format: PixelFormat, width: usize, height: usize, mipmapped: bool) -> Self {
        let raw = unsafe {
            MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
                nil, pixel_format.into(), width as NSUInteger, height as NSUInteger,
                mipmapped as BOOL)
        };
        FromRaw::from_raw(raw).unwrap()
    }

    pub fn new_cube(pixel_format: PixelFormat, size: usize, mipmapped: bool) -> Self {
        let raw = unsafe {
            MTLTextureDescriptor::textureCubeDescriptorWithPixelFormat_size_mipmapped(
                nil, pixel_format.into(), size as NSUInteger, mipmapped as BOOL)
        };
        FromRaw::from_raw(raw).unwrap()
    }

    pub fn texture_type(&self) -> TextureType {
        unsafe { MTLTextureDescriptor::textureType(self.0).into() }
    }

    pub fn set_texture_type(&mut self, texture_type: TextureType) {
        unsafe { self.0.setTextureType(texture_type.into()) }
    }

    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { MTLTextureDescriptor::pixelFormat(self.0).into() }
    }

    pub fn set_pixel_format(&mut self, pixel_format: PixelFormat) {
        unsafe { self.0.setPixelFormat(pixel_format.into()) }
    }

    pub fn width(&self) -> usize {
        unsafe { MTLTextureDescriptor::width(self.0) as usize }
    }

    pub fn set_width(&mut self, width: usize) {
        unsafe { self.0.setWidth(width as NSUInteger) }
    }

    pub fn height(&self) -> usize {
        unsafe { MTLTextureDescriptor::height(self.0) as usize }
    }

    pub fn set_height(&mut self, height: usize) {
        unsafe { self.0.setHeight(height as NSUInteger) }
    }

    pub fn depth(&self) -> usize {
        unsafe { MTLTextureDescriptor::depth(self.0) as usize }
    }

    pub fn set_depth(&mut self, depth: usize) {
        unsafe { self.0.setDepth(depth as NSUInteger) }
    }

    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height(), self.depth())
    }

    pub fn set_size(&mut self, Size {width, height, depth}: Size) {
        self.set_width(width);
        self.set_height(height);
        self.set_depth(depth);
    }

    pub fn mipmap_level_count(&self) -> usize {
        unsafe { MTLTextureDescriptor::mipmapLevelCount(self.0) as usize }
    }

    pub fn set_mipmap_level_count(&mut self, mipmap_level_count: usize) {
        unsafe { self.0.setMipmapLevelCount(mipmap_level_count as NSUInteger) }
    }

    pub fn sample_count(&self) -> usize {
        unsafe { MTLTextureDescriptor::sampleCount(self.0) as usize }
    }

    pub fn set_sample_count(&mut self, sample_count: usize) {
        unsafe { self.0.setSampleCount(sample_count as NSUInteger) }
    }

    pub fn array_length(&self) -> usize {
        unsafe { MTLTextureDescriptor::arrayLength(self.0) as usize }
    }

    pub fn set_array_length(&mut self, array_length: usize) {
        unsafe { self.0.setArrayLength(array_length as NSUInteger) }
    }

    pub fn resource_options(&self) -> ResourceOptions {
        unsafe { self.0.resourceOptions().into() }
    }

    pub fn set_resource_options(&mut self, options: ResourceOptions) {
        unsafe { self.0.setResourceOptions(options.into()) }
    }

    pub fn cpu_cache_mode(&self) -> CpuCacheMode {
        unsafe { self.0.cpuCacheMode().into() }
    }

    pub fn set_cpu_cache_mode(&mut self, cache_mode: CpuCacheMode) {
        unsafe { self.0.setCpuCacheMode(cache_mode.into()) }
    }

    pub fn storage_mode(&self) -> StorageMode {
        unsafe { self.0.storageMode().into() }
    }

    pub fn set_storage_mode(&mut self, storage_mode: StorageMode) {
        unsafe { self.0.setStorageMode(storage_mode.into()) }
    }

    pub fn usage(&self) -> TextureUsage {
        unsafe { MTLTextureDescriptor::usage(self.0).into() }
    }

    pub fn set_usage(&mut self, usage: TextureUsage) {
        unsafe { self.0.setUsage(usage.into()) }
    }
}

impl Clone for TextureDescriptor {
    fn clone(&self) -> Self {
        unsafe { FromRaw::from_raw(self.0.copy()).unwrap() }
    }
}

impl_from_into_raw!(TextureDescriptor, of class "MTLTextureDescriptor");

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
