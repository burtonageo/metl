use cocoa::base::id;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A type which can be constructed from an Objective-C pointer.
pub trait FromRaw: Sized {
    /// Creates the object from the pointer. If there is a problem with the pointer,
    /// this function will return a FromRawError.
    fn from_raw(raw_pointer: id) -> Result<Self, FromRawError>;
}

/// A type which can be borrowed as an Objective-C pointer.
pub trait AsRaw {
    /// Get an immutable reference to the pointer.
    fn as_raw(&self) -> &id;

    /// Get a mutable reference to the pointer.
    fn as_raw_mut(&mut self) -> &mut id;
}

/// A type which can be converted into an Objective-C pointer.
pub trait IntoRaw {
    /// Get the underlying pointer to the object. Releases ownership.
    fn into_raw(self) -> id;
}

/// Represents errors which may occur when constructing an object from a pointer.
#[derive(Clone, Debug)]
pub enum FromRawError {
    /// The pointer was nil.
    NilPointer,

    /// The pointer does not point to either:
    ///
    /// * an object of the correct type,
    /// * an object which does not have a certain base class,
    /// * an object which does not implement a particular protocol.
    WrongPointerType
}

macro_rules! impl_from_into_raw {
    ($wrapper_type:ident, of protocol $protocol:expr) => (
        impl $crate::FromRaw for $wrapper_type {
            fn from_raw(raw_pointer: id) -> Result<Self, $crate::FromRawError> {
                use $crate::internal::conforms_to_protocol;
                use cocoa::base::nil;
                if raw_pointer == nil {
                    Err($crate::FromRawError::NilPointer)
                } else if unsafe { conforms_to_protocol(raw_pointer, $protocol) } {
                    Err($crate::FromRawError::WrongPointerType)
                } else {
                    Ok($wrapper_type(raw_pointer))
                }
            }
        }

        impl $crate::AsRaw for $wrapper_type {
            fn as_raw(&self) -> &id {
                &self.0
            }

            fn as_raw_mut(&mut self) -> &mut id {
                &mut self.0
            }
        }

        impl $crate::IntoRaw for $wrapper_type {
            fn into_raw(self) -> id {
                self.0
            }
        }
    )

    ($wrapper_type:ident, of class $class:expr) => (
        impl $crate::FromRaw for $wrapper_type {
            fn from_raw(raw_pointer: id) -> Result<Self, $crate::FromRawError> {
                use $crate::internal::is_kind_of_class;
                use cocoa::base::nil;
                if raw_pointer == nil {
                    Err($crate::FromRawError::NilPointer)
                } else if unsafe { is_kind_of_class(raw_pointer, $class) } {
                    Err($crate::FromRawError::WrongPointerType)
                } else {
                    Ok($wrapper_type(raw_pointer))
                }
            }
        }
        
        impl $crate::AsRaw for $wrapper_type {
            fn as_raw(&self) -> &id {
                &self.0
            }
        
            fn as_raw_mut(&mut self) -> &mut id {
                &mut self.0
            }
        }
        
        impl $crate::IntoRaw for $wrapper_type {
            fn into_raw(self) -> id {
                self.0
            }
        }
    )
}

impl Display for FromRawError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let descr = match *self {
            FromRawError::NilPointer => "FromRawError::NilPointer",
            FromRawError::WrongPointerType => "FromRawError::WrongPointerType",
        };
        write!(f, "{}", descr)
    }
}

impl Error for FromRawError {
    fn description(&self) -> &str {
        match *self {
            FromRawError::NilPointer => "Attempted to create an object from nil",
            FromRawError::WrongPointerType => "The object pointer is not of the correct class",
        }
    }
}
