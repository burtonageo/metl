use cocoa::base::id;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A type which can be constructed from an Objective-C pointer.
pub trait FromRaw: Sized {
    /// Creates the object from the pointer. If there is a problem with the pointer,
    /// this function will return a FromRawError.
    fn from_raw(raw_pointer: id) -> Result<Self, FromRawError>;
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
