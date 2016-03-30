use block::ConcreteBlock;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSString, NSUInteger};
use error::NSError;
use objc::runtime::YES;
use objc_bringup::NSArray;
use sys::{MTLCopyAllDevices, MTLCreateSystemDefaultDevice, MTLDevice};
use std::convert::From;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;
use std::sync::mpsc;
use sys::MTLFeatureSet;
use {AsRaw, Buffer, CommandQueue, CommandQueueError, CompileOptions, DepthStencilDescriptor,
     DepthStencilState, FromRaw, FromRawError, Library, LibraryError, ResourceOptions,
     SamplerDescriptor, SamplerState, Size, Texture, TextureDescriptor};

pub struct Device(id);

impl Device {
    pub fn system_default_device() -> Result<Self, DeviceError> {
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        if device != nil { Ok(Device(device)) } else { Err(DeviceError::ConstructionFailed) }
    }

    pub fn enumerate_all_system_devices() -> Vec<Result<Self, DeviceError>> {
        let all_devices = unsafe { MTLCopyAllDevices() };
        let mut devices_vec = vec![];

        unsafe {
            for i in 0..all_devices.count() {
                let device = all_devices.objectAtIndex(i);
                if device != nil {
                    devices_vec.push(Ok(Device(device)));
                } else {
                    devices_vec.push(Err(DeviceError::ConstructionFailed));
                }
            }
        }

        devices_vec
    }

    pub fn is_depth24_stencil8_pixel_format_supported(&self) -> bool {
        unsafe { self.0.depth24Stencil8PixelFormatSupported() == YES }
    }

    pub fn is_headless(&self) -> bool {
        unsafe { self.0.headless() == YES }
    }

    pub fn is_low_power(&self) -> bool {
        unsafe { self.0.lowPower() == YES }
    }

    pub fn max_threads_per_group(&self) -> Size {
        unsafe { self.0.maxThreadsPerGroup().into() }
    }

    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_str().unwrap_or(&"") }
    }

    pub fn supports_feature_set(&self, feature_set: FeatureSet) -> bool {
        unsafe { self.0.supportsFeatureSet(feature_set.into()) == YES }
    }

    pub fn supports_texture_sample_count(&self, sample_count: usize) -> bool {
        unsafe { self.0.supportsTextureSampleCount(sample_count as NSUInteger) == YES }
    }

    pub fn new_command_queue(&mut self) -> Result<CommandQueue, CommandQueueError> {
        let command_queue = unsafe { self.0.newCommandQueue() };
        Ok(try!(FromRaw::from_raw(command_queue)))
    }

    pub fn new_command_queue_with_max_buffer_count(&mut self, max_command_buffer_count: usize)
                                                   -> Result<CommandQueue, CommandQueueError> {
        let command_queue = unsafe {
            self.0.newCommandQueueWithMaxCommandBufferCount(max_command_buffer_count as NSUInteger)
        };
        Ok(try!(FromRaw::from_raw(command_queue)))
    }

    // TODO(George): This will panic in the metal lib if not called from a binary in a bundle. Need
    // to add proper error checking
    pub fn new_default_library(&mut self) -> Result<Library, LibraryError> {
        unsafe {
            let library = self.0.newDefaultLibrary();
            Ok(try!(FromRaw::from_raw(library)))
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new_library_with_file(&mut self, file_path: &Path) -> Result<Library, LibraryError> {
        unsafe {
            let path_string = match file_path.as_os_str().to_str() {
                None => unimplemented!(),
                Some(s) => s
            };
            let path = NSString::alloc(nil).init_str(path_string);
            let mut error = nil;
            let library = self.0.newLibraryWithFile_error(path, &mut error);
            if library == nil {
                Err(LibraryError::from(NSError::new(error)))
            } else {
                Ok(try!(FromRaw::from_raw(library)))
            }
        }
    }

    pub fn new_library_with_source(&mut self, source: &str, compile_options: &CompileOptions)
                                   -> Result<Library, LibraryError> {
        unsafe {
            let source = NSString::alloc(nil).init_str(source);
            let options = compile_options.mtl_compile_options();
            let mut error = nil;
            let library = self.0.newLibraryWithSource_options_error(source, options, &mut error);
            if library == nil {
                Err(LibraryError::from(NSError::new(error)))
            } else {
                Ok(try!(FromRaw::from_raw(library)))
            }
        }
    }

    pub fn new_library_with_source_async(&mut self, source: &str, compile_options: &CompileOptions)
                                         -> mpsc::Receiver<Result<Library, LibraryError>> {
        unsafe {
            let source = NSString::alloc(nil).init_str(source);
            let options = compile_options.mtl_compile_options();
            let (sender, receiver) = mpsc::channel();
            let block = ConcreteBlock::new(move |lib, err| {
                if lib != nil {
                    match FromRaw::from_raw(lib) {
                        Ok(library) => sender.send(Ok(library)).unwrap(),
                        Err(e) => sender.send(Err(LibraryError::from(e))).unwrap(),
                    }
                } else {
                    sender.send(Err(LibraryError::from(NSError::new(err)))).unwrap();
                }
            });
            self.0.newLibraryWithSource_options_completionHandler(source, options, &block.copy());
            receiver
        }
    }

    #[allow(unused_variables)]
    // TODO(burtonageo): `data` parameter should be of the correct type
    pub fn new_library_with_data(&mut self, data: ()) -> Result<Library, LibraryError> {
        unimplemented!();
    }

    pub fn new_buffer_with_length(&mut self, length: usize, options: ResourceOptions) -> Buffer {
        unsafe {
            FromRaw::from_raw(self.0.newBufferWithLength_options(length as NSUInteger,
                                                                 options.into()))
                .unwrap()
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn new_buffer_with_bytes(&mut self, bytes: &[u8], options: ResourceOptions)
                                 -> Buffer {
        let len = bytes.len() as NSUInteger;
        let bytes = bytes.as_ptr() as *const _;
        unsafe {
            FromRaw::from_raw(self.0.newBufferWithBytes_length_options(
                bytes, len,options.into())).unwrap()
        }
    }

    pub fn new_buffer_with_bytes_no_copy<'a>(&mut self, bytes: &'a mut [u8],
                                             options: ResourceOptions)
                                             -> BufferRef<'a> {
        let len = bytes.len() as NSUInteger;
        let bytes = bytes.as_ptr() as *mut _;
        let dealloc = ConcreteBlock::new(|ptr, len| {
            use std::slice;
            unsafe { drop(slice::from_raw_parts(ptr as *const u8, len as usize)) }
        });
        let buffer = unsafe {
            FromRaw::from_raw(self.0.newBufferWithBytesNoCopy_length_options_deallocator(bytes, len,
                                                                                         options.into(),
                                                                                         &dealloc)).unwrap()
        };
        BufferRef { buffer: buffer, marker: PhantomData }
    }

    pub fn new_texture(&mut self, descriptor: &TextureDescriptor) -> Result<Texture, FromRawError> {
        unsafe { FromRaw::from_raw(self.0.newTextureWithDescriptor(*descriptor.as_raw())) }
    }

    pub fn new_sampler_state(&mut self, descriptor: &SamplerDescriptor)
                             -> Result<SamplerState, FromRawError> {
        unsafe { FromRaw::from_raw(self.0.newSamplerStateWithDescriptor(*descriptor.as_raw())) }
    }

    pub fn new_depth_stencil_state(&mut self, descriptor: &DepthStencilDescriptor)
                                   -> Result<DepthStencilState, FromRawError> {
        unsafe {
            FromRaw::from_raw(self.0.newDepthStencilStateWithDescriptor(*descriptor.as_raw()))
        }
    }
}

pub struct BufferRef<'a> {
    buffer: Buffer,
    marker: PhantomData<&'a ()>
}

impl<'a> Deref for BufferRef<'a> {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl_from_into_raw!(Device, of protocol "MTLDevice");

#[cfg(target_os = "ios")]
convertible_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum FeatureSet: MTLFeatureSet {
        iOSGpuFamily1_v1 => MTLFeatureSet_iOS_GPUFamily1_v1,
        iOSGpuFamily2_v1 => MTLFeatureSet_iOS_GPUFamily2_v1,
        iOSGpuFamily1_v2 => MTLFeatureSet_iOS_GPUFamily1_v2,
        iOSGpuFamily2_v2 => MTLFeatureSet_iOS_GPUFamily2_v2,
        iOSGpuFamily3_v1 => MTLFeatureSet_iOS_GPUFamily3_v1
    }
}

#[cfg(target_os = "macos")]
convertible_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum FeatureSet: MTLFeatureSet {
        OsxGpuFamily1_v1 => MTLFeatureSet_OSX_GPUFamily1_v1,
        _non_unary_compile_dummy => _non_unary_compile_dummy
    }
}

#[derive(Clone, Debug)]
pub enum DeviceError {
    ConstructionFailed,
    RawError(FromRawError)
}

impl From<FromRawError> for DeviceError {
    fn from(error: FromRawError) -> Self {
        DeviceError::RawError(error)
    }
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let descr = match *self {
            DeviceError::ConstructionFailed => "DeviceError::ConstructionFailed",
            DeviceError::RawError(_) => "DeviceError::ConstructionFailed",
        };
        write!(f, "{}", descr)
    }
}

impl Error for DeviceError {
    fn description(&self) -> &str {
        match *self {
            DeviceError::ConstructionFailed => {
                "Could not create a default device. Please ensure that you are using at least OSX \
                 10.11 or iOS 8.0"
            }
            DeviceError::RawError(_) => "There was an error with the device pointer",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &DeviceError::ConstructionFailed => None,
            &DeviceError::RawError(ref raw_err) => Some(raw_err),
        }
    }
}
