use cocoa::base::{id, nil};
use std::convert::{From, Into};
use std::error::Error;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};

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

impl FromRaw for id {
    fn from_raw(ptr: id) -> Result<Self, FromRawError> {
        Ok(ptr)
    }
}

impl AsRaw for id {
    fn as_raw(&self) -> &id {
        self
    }

    fn as_raw_mut(&mut self) -> &mut id {
        self
    }
}

impl IntoRaw for id {
    fn into_raw(self) -> id {
        self
    }
}

/// An Objective-C pointer which sends the `release` message to its object on drop
#[derive(Eq, PartialEq)]
pub struct StrongPtr(id);

impl fmt::Debug for StrongPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StrongPtr({:p})", self.0)
    }
}

impl fmt::Display for StrongPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(self, f)
    }
}

impl fmt::Pointer for StrongPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}", self.0)
    }
}

impl Deref for StrongPtr {
    type Target = id;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StrongPtr {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl FromRaw for StrongPtr {
    fn from_raw(ptr: id) -> Result<Self, FromRawError> {
        Ok(From::from(ptr))
    }
}

impl AsRaw for StrongPtr {
    fn as_raw(&self) -> &id {
        self.deref()
    }

    fn as_raw_mut(&mut self) -> &mut id {
        self.deref_mut()
    }
}

impl IntoRaw for StrongPtr {
    fn into_raw(self) -> id {
        self.into()
    }
}

impl Clone for StrongPtr {
    fn clone(&self) -> Self {
        let retained = unsafe { msg_send![self.0, retain] };
        StrongPtr(retained)
    }
}

impl Drop for StrongPtr {
    fn drop(&mut self) {
        unsafe { msg_send![self.0, release] };
        self.0 = nil;
    }
}

impl From<id> for StrongPtr {
    fn from(ptr: id) -> Self {
        StrongPtr(ptr)
    }
}

impl Into<id> for StrongPtr {
    /// Get the underlying pointer from `Self`. The pointer will no longer be released on drop.
    fn into(self) -> id {
        let self_ptr = self.0;
        mem::forget(self);
        self_ptr
    }
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
                } else if !conforms_to_protocol(raw_pointer, $protocol) {
                    Err($crate::FromRawError::WrongPointerType)
                } else {
                    Ok($wrapper_type(From::from(raw_pointer)))
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
                self.0.into()
            }
        }
    );

    ($wrapper_type:ident, of class $class:expr) => (
        impl $crate::FromRaw for $wrapper_type {
            fn from_raw(raw_pointer: id) -> Result<Self, $crate::FromRawError> {
                use $crate::internal::is_kind_of_class;
                use cocoa::base::nil;
                if raw_pointer == nil {
                    Err($crate::FromRawError::NilPointer)
                } else if !is_kind_of_class(raw_pointer, $class) {
                    Err($crate::FromRawError::WrongPointerType)
                } else {
                    Ok($wrapper_type(From::from(raw_pointer)))
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
                self.0.into()
            }
        }
    )
}

impl fmt::Display for FromRawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
