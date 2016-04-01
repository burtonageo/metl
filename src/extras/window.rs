use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSMenu, NSMenuItem,
                    NSApplicationActivateIgnoringOtherApps, NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSRunningApplication, NSResizableWindowMask, NSClosableWindowMask, NSFullScreenWindowMask,
                    NSMiniaturizableWindowMask};
use cocoa::base::{nil, NO};
use cocoa::foundation::{NSUInteger, NSAutoreleasePool, NSString, NSProcessInfo, NSRect, NSPoint, NSSize};
use std::convert::Into;
use std::ffi::CStr;
use std::sync::{Once, ONCE_INIT};
use StrongPtr;

pub struct Window(StrongPtr);

impl Window {
    pub fn display(&self) {
    }

    pub fn size(&self) -> (usize, usize) {
        unimplemented!();
    }
}

pub enum WindowPosition {
    Centered,
    AtPoint(f64, f64)
}

pub enum WindowStyle {
    FullScreen,
    Windowed {
        is_closable: bool,
        is_minimizable: bool,
        is_maximizable: bool
    }
}

impl WindowStyle {
    fn into_mask(self) -> NSUInteger {
        match self {
            WindowStyle::FullScreen => NSFullScreenWindowMask as NSUInteger,
            WindowStyle::Windowed {is_closable, is_minimizable, is_maximizable} => {
                let mut style = NSTitledWindowMask as NSUInteger;

                if is_closable {
                    style |= NSClosableWindowMask as NSUInteger;
                }

                if is_minimizable {
                    style |= NSMiniaturizableWindowMask as NSUInteger;
                }

                if is_maximizable {
                    style |= NSResizableWindowMask as NSUInteger;
                }

                style
            }
        }
    }
}

pub struct WindowBuilder {
    pub position: WindowPosition,
    pub size: (f64, f64),
    pub style: WindowStyle,
    pub title: String
}

impl Default for WindowBuilder {
    fn default() -> Self {
        let app_name = unsafe {
            CStr::from_ptr(NSProcessInfo::processInfo(nil).processName().UTF8String())
        };
        let app_name = app_name.to_string_lossy().into_owned();

        WindowBuilder {
            position: WindowPosition::Centered,
            size: (800.0, 600.0),
            style: WindowStyle::Windowed {
                is_closable: true,
                is_minimizable: false,
                is_maximizable: true
            },
            title: app_name
        }
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn window_size(self, size: (f64, f64)) -> Self {
        WindowBuilder { size: size, ..self }
    }

    pub fn style(self, style: WindowStyle) -> Self {
        WindowBuilder { style: style, ..self }
    }

    pub fn title<S: Into<String>>(self, title: S) -> Self {
        WindowBuilder { title: title.into(), ..self }
    }

    pub fn build(self) -> Result<Window, ()> {
        static APP_START: Once = ONCE_INIT;
        APP_START.call_once(|| {
            unsafe {
                let _pool = NSAutoreleasePool::new(nil);
                let app = NSApp();
                app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

                let menubar = NSMenu::new(nil).autorelease();
                let app_menu_item = NSMenuItem::new(nil).autorelease();
                menubar.addItem_(app_menu_item);
                app.setMainMenu_(menubar);

                let app_menu = NSMenu::new(nil).autorelease();
                let quit_prefix = NSString::alloc(nil).init_str("Quit ");
                let quit_title = quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
                let quit_action = sel!(terminate:);
                let quit_key = NSString::alloc(nil).init_str("q");
                let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
                    quit_title,
                    quit_action,
                    quit_key
                ).autorelease();
                app_menu.addItem_(quit_item);
                app_menu_item.setSubmenu_(app_menu);
            }
        });

        unsafe {
            let position = match self.position {
                WindowPosition::Centered => NSPoint::new(0.0, 0.0),
                WindowPosition::AtPoint(x, y) => NSPoint::new(x, y)
            };

            let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                                                  NSRect::new(position, NSSize::new(self.size.0, self.size.1)),
                                                  self.style.into_mask(),
                                                  NSBackingStoreBuffered,
                                                  NO)
                                              .autorelease();
            if let WindowPosition::Centered = self.position {
                window.center();
            }

            window.setTitle_(NSString::alloc(nil).init_str(&self.title));

            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            window.makeKeyAndOrderFront_(current_app);
            NSApp().run();
            Ok(Window(StrongPtr::from(window)))
        }
    }
}
