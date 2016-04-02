use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
                    NSApplicationActivationPolicyRegular, NSBackingStoreBuffered,
                    NSClosableWindowMask, NSFullScreenWindowMask, NSMenu, NSMenuItem,
                    NSMiniaturizableWindowMask, NSResizableWindowMask, NSRunningApplication,
                    NSTitledWindowMask, NSWindow, NSEventType};
use cocoa::base::{BOOL, NO, YES, id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString,
                        NSUInteger};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Protocol, Sel};
use std::convert::Into;
use std::ffi::CStr;
use std::iter::Iterator;
use std::sync::{ONCE_INIT, Once};
use StrongPtr;

fn metl_app_delegate() -> Option<id> {
    const APP_DELEGATE_CLASS_NAME: &'static str = "MetlApplicationDelegate";
    static APP_DELEGATE_DECLARE: Once = ONCE_INIT;

    APP_DELEGATE_DECLARE.call_once(|| {
        let nsobject = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new(APP_DELEGATE_CLASS_NAME, nsobject).unwrap();

        // todo(burtonageo): Can't get NSApplicationDelegate protocol for some reason :-/
        //                   Investigate why

        // let ns_application_delegate_protocol = Protocol::get("NSApplicationDelegate").unwrap();
        // decl.add_protocol(ns_application_delegate_protocol);

        extern "C" fn terminate_after_window_close(_this: &Object, _cmd: Sel, _sender: id) -> BOOL {
            YES
        }

        unsafe {
            let on_window_close: extern "C" fn(&Object, Sel, id) -> BOOL =
                terminate_after_window_close;
            decl.add_method(sel!(applicationShouldTerminateAfterLastWindowClosed:),
                            on_window_close);
        }

        decl.register();
    });

    let cls = Class::get(APP_DELEGATE_CLASS_NAME);
    cls.map(|cls| unsafe { msg_send![cls, new] })
}

pub struct Window(StrongPtr);

impl Window {
    pub fn events(&self) -> Events {
        unsafe {
            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            self.0.makeKeyAndOrderFront_(current_app);
            NSApp().run();
            Events { window: &self }
        }
    }
}

pub struct Events<'a> {
    window: &'a Window
}

pub enum KeyPress {
    PressDown,
    PressUp,
    Repeat
}

pub enum Event {
    Idle,
    Display,
    KeyBoard(KeyPress, char),
    Mouse
}

impl<'a> Iterator for Events<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        let is_running: BOOL = unsafe { msg_send![NSApp(), running] };
        if is_running == NO {
            return None
        }

        let current_event: id = unsafe {
            let win_ptr: id = *self.window.0;
            msg_send![win_ptr, currentEvent]
        };

        if current_event.is_null() {
            Some(Event::Idle)
        } else {
            // blah
            Some(Event::Display)
        }
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
                let delegate = metl_app_delegate().unwrap_or(nil);
                app.setDelegate_(delegate);
                app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

                let menubar = NSMenu::new(nil).autorelease();
                let app_menu_item = NSMenuItem::new(nil).autorelease();
                menubar.addItem_(app_menu_item);
                app.setMainMenu_(menubar);

                let app_menu = NSMenu::new(nil).autorelease();
                let quit_prefix = NSString::alloc(nil).init_str("Quit ");
                let quit_title =
                    quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil)
                                                             .processName());
                let quit_key = NSString::alloc(nil).init_str("q");
                let quit_item = NSMenuItem::alloc(nil)
                                    .initWithTitle_action_keyEquivalent_(quit_title,
                                                                         sel!(terminate:),
                                                                         quit_key)
                                    .autorelease();
                app_menu.addItem_(quit_item);
                app_menu_item.setSubmenu_(app_menu);
            }
        });

        unsafe {
            let position = match self.position {
                WindowPosition::Centered => NSPoint::new(0.0, 0.0),
                WindowPosition::AtPoint(x, y) => NSPoint::new(x, y),
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

            // TODO(burtonageo): Add metal view here

            Ok(Window(StrongPtr::from(window)))
        }
    }
}
