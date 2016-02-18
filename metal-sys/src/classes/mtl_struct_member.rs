use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use MTLDataType;

/// `MTLStructMember` is part of the reflection API that allows Metal framework
/// code to query details about an argument of a Metal shading language
/// function. A `MTLStructMember` object describes the data type of one field
/// in a struct that is passed as a `MTLFunction` argument, which is represented
/// by `MTLArgument`.
///
/// Your app does not create a `MTLStructMember` object directly. A
/// `MTLStructMember` object is either obtained from the members property or the
/// `memberByName:` method of a `MTLStructType` object. You examine the dataType
/// property of the `MTLStructMember` object, which may indicate the struct
/// member is another struct (`MTLDataTypeStruct`), an array
/// (`MTLDataTypeArray`), or some other type. You recursively drill down every
/// struct member until you reach a data type that is neither a struct nor an array.
pub trait MTLStructMember {
    /// The name of the struct member. (read-only)
    unsafe fn name(self) -> id;

    /// The data type of the struct member. (read-only)
    ///
    /// #Discussion
    ///
    /// For information on possible values, see `MTLDataType`. If the value is
    /// `MTLDataTypeArray`, then the `arrayType` method returns an object that
    /// describes the underlying array. If the value is `MTLDataTypeStruct`,
    /// then the `structType` method returns an object that describes the
    /// underlying struct.
    unsafe fn dataType(self) -> MTLDataType;

    /// The location of this member relative to the start of its struct, in bytes.
    /// (read-only)
    unsafe fn offset(self) -> NSUInteger;

    /// If a struct member holds an array, returns an object that describes the
    /// underlying array.
    unsafe fn arrayType(self) -> id;

    /// If a struct member holds another struct, returns an object that describes
    /// the underlying struct.
    unsafe fn structType(self) -> id;
}

impl MTLStructMember for id {
    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn dataType(self) -> MTLDataType {
        msg_send![self, dataType]
    }

    unsafe fn offset(self) -> NSUInteger {
        msg_send![self, offset]
    }

    unsafe fn arrayType(self) -> id {
        msg_send![self, arrayType]
    }

    unsafe fn structType(self) -> id {
        msg_send![self, structType]
    }
}
