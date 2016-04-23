use cocoa::base::{class, id};
use objc::runtime::{BOOL, Class, Protocol, YES};

macro_rules! convertible_enum {
    ($(#[$top_lvl_attrs:meta])* pub enum $enum_nm:ident : $convert:ident {
        $($(#[$arm_attrs:meta])* $arm:ident => $other:ident),*
    }) => (
        $(#[$top_lvl_attrs])*
        pub enum $enum_nm {
            $($(#[$arm_attrs])* $arm),*
        }

        convertible_enum!(@_impl conversion from $enum_nm to $convert {
            $($arm => $other),*
        });
    );

    ($(#[$top_lvl_attrs:meta])* enum $enum_nm:ident : $convert:ident {
        $($(#[$arm_attrs:meta])* $arm:ident => $other:ident),*
    }) => (
        $(#[$top_lvl_attrs])*
        enum $enum_nm {
            $($(#[$arm_attrs])* $arm),*
        }

        convertible_enum!(@_impl conversion from $enum_nm to $convert {
            $($arm => $other),*
        });
    );

    (@_impl conversion from $enum_nm:ident to $convert:ident {
        $($arm:ident => $other:ident),*
    }) => (
        impl ::std::convert::Into<$convert> for $enum_nm {
            fn into(self) -> $convert {
                match self {
                    $($enum_nm::$arm => $convert::$other),*
                }
            }
        }

        impl ::std::convert::From<$convert> for $enum_nm {
            fn from(other: $convert) -> Self {
                match other {
                    $($convert::$other => $enum_nm::$arm),*
                }
            }
        }
    );
}

pub fn conforms_to_protocol(object: id, protocol_name: &str) -> bool {
    match Protocol::get(protocol_name) {
        Some(protocol) => unsafe { &*object }.class().conforms_to(protocol),
        None => false
    }
}

pub fn is_kind_of_class(object: id, class_name: &str) -> bool {
    match Class::get(class_name) {
        None => false,
        Some(class) => {
            let is_class: BOOL = unsafe {
                msg_send![object, isKindOfClass:class]
            };
            is_class == YES
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cocoa::base::nil;
    use cocoa::foundation::NSString;

    #[test]
    fn test_conforms_to_protocol() {
        unsafe {
            let nsstr = NSString::alloc(nil).init_str("Hello, world");
            assert!(conforms_to_protocol(nsstr, "NSCopying"));
        }
    }

    #[test]
    fn test_doesnt_conform_to_protocol() {
        unsafe {
            let nsstr = NSString::alloc(nil).init_str("blah");
            assert!(!conforms_to_protocol(nsstr, "MTLDevice"));
        }
    }

    #[test]
    fn test_is_kind_of_class() {
        let nsstr = unsafe { NSString::alloc(nil).init_str("Hello, world") };
        assert!(is_kind_of_class(nsstr, "NSString"));
    }

    #[test]
    fn test_is_not_kind_of_class() {
        use sys::MTLCompileOptions;
        let compile_opts = unsafe { MTLCompileOptions::new(nil) };
        assert!(!is_kind_of_class(compile_opts, "NSString"));
    }
}
