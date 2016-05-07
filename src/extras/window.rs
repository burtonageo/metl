use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use cocoa::appkit::NSWindow;
use core_graphics::geometry::{CGRect, CGPoint, CGSize};
use objc::runtime::{BOOL, Class, YES};
use raw::{AsRaw, FromRaw, FromRawError, IntoRaw};
use std::error::Error;
use std::fmt;
use std::marker;
use std::ops::{Deref, DerefMut};
use sys::{MTLClearColor, MTLPixelFormat};
use winit;
use winit::os::macos::WindowExt;
use {ClearColor, Device, PixelFormat, RenderPassDescriptor, Texture};

pub use winit::{
    CursorState,
    Event,
    MonitorId,
    MouseCursor,
    PollEventsIterator,
    WaitEventsIterator,
    ElementState,
    MouseButton,
    MouseScrollDelta,
    Touch,
    TouchPhase,
    ScanCode,
    VirtualKeyCode
};

pub struct WindowBuilder {
    window: winit::WindowBuilder,
    device: Device
}

#[derive(Debug)]
pub enum CreationError {
    ViewCreation(ViewError),
    Winit(winit::CreationError)
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::ViewCreation(ref e) => e.description(),
            CreationError::Winit(ref e) => e.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CreationError::ViewCreation(ref e) => Some(e),
            CreationError::Winit(ref e) => Some(e)
        }
    }
}

impl From<winit::CreationError> for CreationError {
    fn from(error: winit::CreationError) -> Self {
        CreationError::Winit(error)
    }
}

impl From<ViewError> for CreationError {
    fn from(error: ViewError) -> Self {
        CreationError::ViewCreation(error)
    }
}

impl WindowBuilder {
    /// Initializes a new `WindowBuilder` with default values.
    #[inline]
    pub fn new(device: Device) -> Self {
        WindowBuilder {
            window: winit::WindowBuilder::new(),
            device: device
        }
    }

    /// Requests the window to be of specific dimensions.
    ///
    /// Width and height are in pixels.
    #[inline]
    pub fn with_dimensions(self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {window: self.window.with_dimensions(width, height), ..self}
    }

    /// Sets a minimum dimension size for the window
    ///
    /// Width and height are in pixels.
    #[inline]
    pub fn with_min_dimensions(self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {window: self.window.with_min_dimensions(width, height), ..self}
    }

    /// Sets a maximum dimension size for the window
    ///
    /// Width and height are in pixels.
    #[inline]
    pub fn with_max_dimensions(self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {window: self.window.with_max_dimensions(width, height), ..self}
    }

    /// Requests a specific title for the window.
    #[inline]
    pub fn with_title(self, title: String) -> WindowBuilder {
        WindowBuilder {window: self.window.with_title(title), ..self}
    }

    /// Requests fullscreen mode.
    ///
    /// If you don't specify dimensions for the window, it will match the monitor's.
    #[inline]
    pub fn with_fullscreen(self, monitor: MonitorId) -> WindowBuilder {
        WindowBuilder {window: self.window.with_fullscreen(monitor), ..self}
    }

    /// Sets whether the window will be initially hidden or visible.
    #[inline]
    pub fn with_visibility(self, visible: bool) -> WindowBuilder {
        WindowBuilder {window: self.window.with_visibility(visible), ..self}
    }

    /// Sets whether the background of the window should be transparent.
    #[inline]
    pub fn with_transparency(self, transparent: bool) -> WindowBuilder {
        WindowBuilder {window: self.window.with_transparency(transparent), ..self}
    }

    /// Sets whether the window should have a border, a title bar, etc.
    #[inline]
    pub fn with_decorations(self, decorations: bool) -> WindowBuilder {
        WindowBuilder {window: self.window.with_decorations(decorations), ..self}
    }

    /// Enables multitouch
    #[inline]
    pub fn with_multitouch(self) -> WindowBuilder {
        WindowBuilder {window: self.window.with_multitouch(), ..self}
    }

    /// Builds the window.
    ///
    /// Error should be very rare and only occur in case of permission denied, incompatible system,
    /// out of memory, etc.
    pub fn build(self) -> Result<Window, CreationError> {
        let window = try!(self.window.build());
        let view = try!(View::new(&window, self.device));

        unsafe {
            let native_window: id = window.get_nswindow() as id;
            native_window.setContentView_(view.mtk_view.0);
        }

        Ok(Window {
            window: window,
            view: view
        })
    }

    /// Builds the window.
    ///
    /// The context is build in a *strict* way. That means that if the backend couldn't give
    /// you what you requested, an `Err` will be returned.
    #[inline]
    pub fn build_strict(self) -> Result<Window, CreationError> {
        self.build()
    }
}

pub struct Window {
    window: winit::Window,
    pub view: View
}

impl Window {
    pub fn poll_events(&self) -> PollEventsIterator {
        self.window.poll_events()
    }

    pub fn wait_events(&self) -> WaitEventsIterator {
        self.window.wait_events()
    }

    pub fn window(&self) -> Ref<winit::Window> {
        Ref(&self.window)
    }
}

pub struct View {
    mtk_view: MtkView
}

struct MtkView(id);
impl_from_into_raw!(MtkView, of class "MTKView");

impl View {
    fn new(window: &winit::Window, device: Device) -> Result<Self, ViewError> {
        let window_origin = {
            let (x, y) = try!(window.get_position().ok_or(ViewError::no_window()));
            CGPoint { x: x as f64, y: y as f64 }
        };

        let window_size = {
            let (w, h) = try!(window.get_inner_size().ok_or(ViewError::no_window()));
            CGSize { width: w as f64, height: h as f64 }
        };

        let view_frame = CGRect { origin: window_origin, size: window_size };

        let view_object: id = unsafe {
            // Dummy function to ensure that we link against MetalKit
            #[allow(dead_code)]
            #[link(name = "MetalKit", kind = "framework")]
            extern "C" {
                fn MTKModelIOVertexDescriptorFromMetal(metalDescriptor: id) -> id;
            }

            let view_class = Class::get("MTKView").expect("Could not find MTKView class");
            let view: id = msg_send![view_class, alloc];
            msg_send![view, initWithFrame:view_frame
                                   device:device.into_raw()]
        };

        Ok(View {
            mtk_view: try!(MtkView::from_raw(view_object)),
        })
    }

    fn raw_view(&self) -> id {
        self.mtk_view.0
    }

    pub fn device<'a>(&self) -> Option<PhantomRef<'a, Device>> {
        let device = unsafe {
            Device::from_raw(msg_send![*self.mtk_view.as_raw(), device])
        };
        device.ok().map(|d| PhantomRef(d, marker::PhantomData))
    }

    pub fn device_mut<'a>(&mut self) -> Option<PhantomRefMut<'a, Device>> {
        let device = unsafe {
            Device::from_raw(msg_send![*self.mtk_view.as_raw(), device])
        };
        device.ok().map(|d| PhantomRefMut(d, marker::PhantomData))
    }

    pub fn clear_color(&self) -> ClearColor {
        let color: MTLClearColor = unsafe {
            msg_send![*self.mtk_view.as_raw(), clearColor]
        };
        color.into()
    }

    pub fn set_clear_color<C: Into<MTLClearColor>>(&mut self, clear_color: C) {
        unsafe { msg_send![*self.mtk_view.as_raw(), setClearColor:clear_color.into()] }
    }

    pub fn clear_depth(&self) -> f64 {
        unsafe { msg_send![*self.mtk_view.as_raw(), clearDepth] }
    }

    pub fn set_clear_depth(&mut self, clear_depth: f64) {
        unsafe { msg_send![*self.mtk_view.as_raw(), setClearDepth:clear_depth] }
    }

    pub fn clear_stencil(&self) -> u32 {
        unsafe { msg_send![*self.mtk_view.as_raw(), clearStencil] }
    }

    pub fn set_clear_stencil(&mut self, clear_stencil: u32) {
        unsafe { msg_send![*self.mtk_view.as_raw(), setClearStencil:clear_stencil] }
    }

    pub fn color_pixel_format(&self) -> PixelFormat {
        let pixel_format: MTLPixelFormat = unsafe {
            msg_send![*self.mtk_view.as_raw(), colorPixelFormat]
        };
        pixel_format.into()
    }

    pub fn set_color_pixel_format<P: Into<MTLPixelFormat>>(&mut self, color_pixel_format: P) {
        unsafe { msg_send![*self.mtk_view.as_raw(), setColorPixelFormat:color_pixel_format.into()] }
    }

    pub fn depth_stencil_pixel_format(&self) -> PixelFormat {
        let pixel_format: MTLPixelFormat = unsafe {
            msg_send![*self.mtk_view.as_raw(), depthStencilPixelFormat]
        };
        pixel_format.into()
    }

    pub fn set_depth_stencil_pixel_format<P: Into<MTLPixelFormat>>(&mut self, depth_stencil_pixel_format: P) {
        unsafe { msg_send![*self.mtk_view.as_raw(), setDepthStencilPixelFormat:depth_stencil_pixel_format.into()] }
    }

    pub fn sample_count(&self) -> usize {
        let sample_count: NSUInteger = unsafe {
            msg_send![*self.mtk_view.as_raw(), sampleCount]
        };
        sample_count as usize
    }

    pub fn set_sample_count(&mut self, sample_count: usize) {
        let sample_count = sample_count as NSUInteger;
        unsafe { msg_send![*self.mtk_view.as_raw(), setSampleCount:sample_count] }
    }

    pub fn current_render_pass_descriptor(&self) -> Option<PhantomRef<RenderPassDescriptor>> {
        let descriptor: id = unsafe { msg_send![self.raw_view(), currentRenderPassDescriptor] };
        FromRaw::from_raw(descriptor).ok().map(|d| PhantomRef(d, marker::PhantomData))
    }

    pub fn current_depth_stencil_texture(&self) -> Option<PhantomRef<Texture>> {
        let texture: id = unsafe { msg_send![self.raw_view(), depthStencilTexture] };
        FromRaw::from_raw(texture).ok().map(|t| PhantomRef(t, marker::PhantomData))
    }

    pub fn multisample_color_texture(&self) -> Option<PhantomRef<Texture>> {
        let texture: id = unsafe { msg_send![self.raw_view(), multisampleColorTexture] };
        FromRaw::from_raw(texture).ok().map(|t| PhantomRef(t, marker::PhantomData))
    }

    pub fn preferred_frames_per_second(&self) -> usize {
        let fps: NSUInteger = unsafe { msg_send![self.raw_view(), sampleCount] };
        fps as usize
    }

    pub fn set_preferred_frames_per_second(&self, preferred_fps: usize) {
        let preferred_fps = preferred_fps as NSUInteger;
        unsafe { msg_send![self.raw_view(), setPreferredFramesPerSecond:preferred_fps] }
    }

    pub fn is_paused(&self) -> bool {
        let is_paused: BOOL = unsafe { msg_send![self.raw_view(), isPaused] };
        is_paused == YES
    }

    pub fn draw(&self) {
        unsafe { msg_send![self.raw_view(), draw] }
    }
}

#[derive(Clone, Debug)]
pub struct ViewError(ViewErrorInner);

#[derive(Clone, Debug)]
enum ViewErrorInner {
    NoWindow,
    CreateError(FromRawError)
}

impl ViewError {
    fn no_window() -> Self {
        ViewError(ViewErrorInner::NoWindow)
    }
}

impl fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ViewError {
    fn description(&self) -> &str {
        match self.0 {
            ViewErrorInner::NoWindow => "The native window no longer exists",
            ViewErrorInner::CreateError(_) => "There was an issue creating the view",
        }
    }

    fn cause(&self) -> Option<&Error> {
        if let ViewErrorInner::CreateError(ref e) = self.0 {
            Some(e)
        } else {
            None
        }
    }
}

impl From<FromRawError> for ViewError {
    fn from(error: FromRawError) -> Self {
        ViewError(ViewErrorInner::CreateError(error))
    }
}

pub struct Ref<'a, T: 'a>(&'a T);

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub struct PhantomRef<'a, T: 'a>(T, marker::PhantomData<&'a ()>);

impl<'a, T> Deref for PhantomRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct PhantomRefMut<'a, T: 'a>(T, marker::PhantomData<&'a mut ()>);

impl<'a, T> Deref for PhantomRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for PhantomRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

pub trait WindowBuilderExt {
    fn build_metal(self, device: Device) -> Result<MetalFacade, CreationError>;
}

impl WindowBuilderExt for WindowBuilder {
    fn build_metal(self, device: Device) -> Result<MetalFacade, CreationError> {
        let window = try!(self.build());
        Ok(MetalFacade {
            window: window,
            device: device
        })
    }
}

pub struct MetalFacade {
    window: Window,
    device: Device
}

impl MetalFacade {
    #[inline]
    pub fn window(&self) -> &Window {
        &self.window
    }

    #[inline]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    #[inline]
    pub fn device(&self) -> &Device {
        &self.device
    }

    #[inline]
    pub fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    #[inline]
    pub fn poll_events(&self) -> PollEventsIterator {
        self.window.poll_events()
    }

    #[inline]
    pub fn wait_events(&self) -> WaitEventsIterator {
        self.window.wait_events()
    }
}
