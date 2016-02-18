use cocoa::base::{class, id, nil};
use cocoa::foundation::NSString;
use objc::runtime::{BOOL, NO, YES};

macro_rules! convertible_enum {
    ($(#[$top_lvl_attrs:meta])* enum $enum_nm:ident : $convert:ident {
        $($(#[$arm_attrs:meta])* $arm:ident => $other:ident),*
    }) => (
        $(#[$top_lvl_attrs])*
        pub enum $enum_nm {
            $($(#[$arm_attrs])* $arm),*
        }
        
        impl $crate::std::convert::Into<$convert> for $enum_nm {
            fn into(self) -> $convert {
                match self {
                    $($enum_nm::$arm => $convert::$other),*
                }
            }
        }

        impl $crate::std::convert::From<$convert> for $enum_nm {
            fn from(other: $convert) -> Self {
                match other {
                    $($convert::$other => $enum_nm::$arm),*
                }
            }
        }
    )
}

pub unsafe fn conforms_to_protocol(object: id, protocol_name: &str) -> bool {
    #[link(name = "Foundation", kind = "framework")]
    extern "C" {
        fn NSProtocolFromString(namestr: id) -> id;
    }

    let protocol_name_nsstr = NSString::alloc(nil).init_str(protocol_name);
    let protocol = NSProtocolFromString(protocol_name_nsstr);
    let does_conform: BOOL = msg_send![object, conformsToProtocol:protocol];
    does_conform == NO // TODO(George): Investigate why I need to compare to NO rather then YES
}

pub unsafe fn is_kind_of_class(object: id, class_name: &str) -> bool {
    let class = class(class_name);
    let is_kind_of_class: BOOL = msg_send![object, isKindOfClass:class];
    is_kind_of_class == YES
}

#[test]
fn test_conforms_to_protocol() {
    unsafe {
        let nsstr = NSString::alloc(nil).init_str("Hello, world");
        assert!(conforms_to_protocol(nsstr, "NSObjectProtocol"));
    }
}

#[test]
fn test_is_kind_of_class() {
    unsafe {
        let nsstr = NSString::alloc(nil).init_str("Hello, world");
        assert!(is_kind_of_class(nsstr, "NSString"));
    }
}

#[test]
fn test_is_not_kind_of_class() {
    use ::sys::MTLCompileOptions;
    unsafe {
        let compile_opts = MTLCompileOptions::new(nil);
        assert!(!is_kind_of_class(compile_opts, "NSString"));
    }
}
