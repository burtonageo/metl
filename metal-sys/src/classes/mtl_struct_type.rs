use cocoa::base::id;

/// `MTLStructType` is part of the reflection API that allows Metal framework
/// code to query details of a struct that is passed as an argument of a Metal
/// shading language function. You do not allocate or initialize `MTLStructType`
/// objects. To obtain a `MTLStructType` object, you can query the
/// bufferStructType property of a `MTLArgument` object, or call the structType
/// method for a `MTLStructMember` object. To examine the details of the struct,
/// you can recursively drill down the members property of the `MTLStructType`
/// object, which contains details about struct members, each of which is
/// represented by a `MTLStructMember` object.
pub trait MTLStructType {
    /// An array of objects that describe the fields in the struct. (read-only)
    ///
    /// #Description
    ///
    /// Each array element in `members` is a `MTLStructMember` object that
    /// corresponds to one of the fields in the struct.
    unsafe fn members(self) -> id;

    /// Returns an object that represents the member of the struct with a given name.
    ///
    /// #Description
    ///
    /// An object that represents the named struct member. If `name` does not match
    /// a member name, `nil` is returned.
    unsafe fn memberByName(self, name: id) -> id;
}

impl MTLStructType for id {
    unsafe fn members(self) -> id {
        msg_send![self, members] }
    }

    unsafe fn memberByName(self, name: id) -> id {
        msg_send![self, memberByName:name]
    }
}
