use cocoa::appkit::*;
use cocoa::base::{BOOL, NO, YES, id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSDate, NSDefaultRunLoopMode, NSPoint, NSProcessInfo,
                        NSRect, NSSize, NSString, NSUInteger};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use std::collections::VecDeque;
use std::convert::Into;
use std::ffi::CStr;
use std::iter::Iterator;
use std::os::raw::c_void;
use std::sync::{Mutex, ONCE_INIT, Once};
use extras::events::{Event, MouseButton};
use StrongPtr;

macro_rules! autorelease {
    ($e:expr) => ({
        let pool: id = NSAutoreleasePool::new(nil);
        let result = $e;
        let _: () = msg_send![pool, release];
        result
    })
}

fn metl_app_delegate() -> Option<id> {
    const APP_DELEGATE_CLASS_NAME: &'static str = "MetlApplicationDelegate";
    static APP_DELEGATE_DECLARE: Once = ONCE_INIT;

    APP_DELEGATE_DECLARE.call_once(|| {
        let nsobject = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new(APP_DELEGATE_CLASS_NAME, nsobject).unwrap();

        extern "C" fn terminate_after_window_close(_this: &Object, _cmd: Sel, _sender: id) -> BOOL {
            YES
        }

        unsafe {
            let on_window_close: extern "C" fn(&Object, Sel, id) -> BOOL =
                terminate_after_window_close;
            decl.add_method(sel!(applicationShouldTerminateAfterLastWindowClosed:),
                            on_window_close);

            decl.add_ivar::<*mut c_void>("MetlState");
        }

        decl.register();
    });

    Class::get(APP_DELEGATE_CLASS_NAME).map(|cls| unsafe { msg_send![cls, new] })
}

struct WindowDelegate {
    state: Box<DelegateState>,
    _this: StrongPtr
}

impl WindowDelegate {
    fn metl_window_delegate() -> Option<id> {
        const WIN_DELEGATE_CLASS_NAME: &'static str = "MetlWindowDelegate";
        static WIN_DELEGATE_DECLARE: Once = ONCE_INIT;

        WIN_DELEGATE_DECLARE.call_once(|| {
            let nsobject = Class::get("NSObject").unwrap();
            let mut decl = ClassDecl::new(WIN_DELEGATE_CLASS_NAME, nsobject).unwrap();

            extern "C" fn window_should_close(this: &Object, _: Sel, _: id) -> BOOL {
                unsafe {
                    let state: *mut c_void = *this.get_ivar("metlState");
                    let state = state as *mut DelegateState;
                    (*state).pending_events.lock().unwrap().push_back(Event::Closed);
                }
                YES
            }

            extern "C" fn window_did_resize(this: &Object, _: Sel, _: id) {
                unsafe {
                    let state: *mut c_void = *this.get_ivar("metlState");
                    let state = state as *mut DelegateState;
                    let context = *(*state).context;
                    msg_send![context, update];

                    if let Some(_handler) = (*state).resize_handler {
                        // TODO
                    }
                }
            }

            extern "C" fn window_did_become_key(this: &Object, _: Sel, _: id) {
                unsafe {
                    let state: *mut c_void = *this.get_ivar("metlState");
                    let state = state as *mut DelegateState;
                    (*state).pending_events.lock().unwrap().push_back(Event::Focused(true));
                }
            }

            extern "C" fn window_did_resign_key(this: &Object, _: Sel, _: id) {
                unsafe {
                    let state: *mut c_void = *this.get_ivar("metlState");
                    let state = state as *mut DelegateState;
                    (*state).pending_events.lock().unwrap().push_back(Event::Focused(false));
                }
            }

            unsafe {
                let window_should_close: extern "C" fn(&Object, Sel, id) -> BOOL =
                    window_should_close;
                let window_did_resize: extern "C" fn(&Object, Sel, id) = window_did_resize;
                let window_did_become_key: extern "C" fn(&Object, Sel, id) = window_did_become_key;
                let window_did_resign_key: extern "C" fn(&Object, Sel, id) = window_did_resign_key;

                decl.add_method(sel!(windowShouldClose:), window_should_close);
                decl.add_method(sel!(windowDidResize:), window_did_resize);
                decl.add_method(sel!(windowDidBecomeKey:), window_did_become_key);
                decl.add_method(sel!(windowDidResignKey:), window_did_resign_key);

                decl.add_ivar::<*mut c_void>("metlState");
            }

            decl.register();
        });

        Class::get(WIN_DELEGATE_CLASS_NAME).map(|cls| unsafe { msg_send![cls, new] })
    }

    fn new(state: DelegateState) -> Self {
        let mut state = Box::new(state);
        let state_ptr: *mut _ = &mut *state;
        let mut delegate = StrongPtr::from(WindowDelegate::metl_window_delegate().unwrap());
        unsafe {
            (&mut **delegate).set_ivar("metlState", state_ptr as *mut ::std::os::raw::c_void);
            msg_send![*state.window, setDelegate:*delegate];
            WindowDelegate { state: state, _this: delegate }
        }
    }
}

impl Drop for WindowDelegate {
    fn drop(&mut self) {
        unsafe { msg_send![*self.state.window, setDelegate:nil] };
    }
}

#[allow(dead_code)]
struct DelegateState {
    context: StrongPtr,
    view: StrongPtr,
    window: StrongPtr,
    resize_handler: Option<fn(u32, u32)>,
    pending_events: Mutex<VecDeque<Event>>
}

#[allow(dead_code)]
pub struct Window {
    window: StrongPtr,
    delegate: WindowDelegate,
    app: id
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Window {
    pub fn poll_events(&self) -> PollEventsIter {
        PollEventsIter { window: &self }
    }
}

impl Event {
    #[allow(non_snake_case, unused_variables)]
    fn from_native(window: &Window, event: id) -> Option<Self> {
        use ::extras::events::ElementState::*;
        use ::extras::events::Event::*;

        if event.is_null() {
            return None;
        }
        let event_type = unsafe { event.eventType() };
        unsafe {
            NSApp().sendEvent_(if let NSKeyDown = event_type { nil } else { event });
        }

        match event_type {
            NSLeftMouseDown => Some(MouseInput(Pressed, MouseButton::Left)),
            NSLeftMouseUp => Some(MouseInput(Released, MouseButton::Left)),
            NSRightMouseDown => Some(MouseInput(Pressed, MouseButton::Right)), 
            NSRightMouseUp => Some(MouseInput(Released, MouseButton::Right)), 
            NSMouseMoved | NSLeftMouseDragged | NSOtherMouseDragged | NSRightMouseDragged => {
                Some(Refresh)
            }
            NSKeyDown => Some(Refresh),
            NSKeyUp => Some(Refresh),
            NSFlagsChanged => Some(Refresh),
            NSScrollWheel => Some(Refresh),
            _ => None,
            // NSTypePressure => {
            // unsafe { Some(TouchpadPressure(event.pressure(), event.stage())) }
            // }
            //
        }
    }
}

pub struct PollEventsIter<'a> {
    window: &'a Window
}

impl<'a> Iterator for PollEventsIter<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ev) = self.window.delegate.state.pending_events.lock().unwrap().pop_front() {
            return Some(ev);
        }

        let current_event: id = unsafe {
            autorelease! {
                self.window.app.nextEventMatchingMask_untilDate_inMode_dequeue_(
                    NSAnyEventMask.bits() | NSEventMaskPressure.bits(),
                    NSDate::distantPast(nil),
                    NSDefaultRunLoopMode,
                    YES)
            }
        };

        Event::from_native(self.window, current_event)
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
                style |= NSBorderlessWindowMask as NSUInteger;

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
        let app = unsafe { NSApp() };

        APP_START.call_once(|| {
            unsafe {
                autorelease! {{
                    let delegate = metl_app_delegate().unwrap_or(nil);
                    app.setDelegate_(delegate);
                    app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
                    app.finishLaunching();
/*
                    let menubar: id = NSMenu::new(nil).autorelease();
                    let app_menu_item = NSMenuItem::new(nil).autorelease();
                    menubar.addItem_(app_menu_item);
    
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
                    app.setMainMenu_(menubar);
                    app.run();
*/                }}
            }
        });

        let window = unsafe {
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
            window.setAcceptsMouseMovedEvents_(YES);

            app.activateIgnoringOtherApps_(YES);
            window.makeKeyAndOrderFront_(nil);
            StrongPtr::from(window)
        };

        let ds = DelegateState {
            context: StrongPtr::from(nil), // TODO
            view: StrongPtr::from(nil), // TODO
            window: window.clone(),
            resize_handler: None, // TODO
            pending_events: Mutex::new(VecDeque::new())
        };

        Ok(Window {
            window: window,
            delegate: WindowDelegate::new(ds),
            app: app
        })
    }
}
