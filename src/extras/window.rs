use cocoa::base::id;
use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::ptr;
use winit;
use Device;

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
    window: winit::WindowBuilder
}

#[derive(Debug)]
pub enum CreationError {
    NoDevice,
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
            CreationError::NoDevice => "A device was not set for the windowbuilder",
            CreationError::Winit(ref e) => e.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        if let CreationError::Winit(ref e) = *self {
            Some(e)
        } else {
            None
        }
    }
}

impl From<winit::CreationError> for CreationError {
    fn from(error: winit::CreationError) -> Self {
        CreationError::Winit(error)
    }
}

impl WindowBuilder {
    /// Initializes a new `WindowBuilder` with default values.
    #[inline]
    pub fn new() -> Self {
        WindowBuilder {
            window: winit::WindowBuilder::new()
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
        let native_window: id = unsafe { window.platform_window() as id };
        Ok(Window { 
            window: window,
            view: ptr::null_mut()
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
    view: id
}

pub struct WinRef<'a>(&'a winit::Window);

impl<'a> Deref for WinRef<'a> {
    type Target = winit::Window;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Window {
    pub fn poll_events(&self) -> PollEventsIterator {
        self.window.poll_events()
    }

    pub fn wait_events(&self) -> WaitEventsIterator {
        self.window.wait_events()
    }

    pub fn window(&self) -> WinRef {
        WinRef(&self.window)
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
    pub window: Window,
    pub device: Device
}

impl MetalFacade {
    #[inline]
    pub fn poll_events(&self) -> PollEventsIterator {
        self.window.poll_events()
    }

    #[inline]
    pub fn wait_events(&self) -> WaitEventsIterator {
        self.window.wait_events()
    }
}
