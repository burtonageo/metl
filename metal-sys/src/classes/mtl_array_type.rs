use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use objc::runtime::BOOL;
use MTLDataType;

/// A `MTLArrayType` object provides details about a `MTLStructMember` object
/// that is an array. Your app does not create a `MTLArrayType` or
/// `MTLStructMember` object directly. A reflection object,
/// `MTLRenderPipelineReflection` or `MTLComputePipelineReflection`, can contain
/// a `MTLStructMember` object, if one of its arguments is a struct (that is,
/// if the `bufferDataType` property of `MTLArgument` is `MTLDataTypeStruct`).
/// If the `dataType` property of `MTLStructMember` is `MTLDataTypeArray`, then
/// the `MTLStructMember` object is an array, and the `arrayType` method of
/// `MTLStructMember` returns a `MTLArrayType` object that contains array
/// details such as length, stride, and element type.
pub trait MTLArrayType {
    /// The number of elements in the array. (read-only)
    unsafe fn arrayLength(self) -> NSUInteger;
    
    /// The data type of the array’s elements. (read-only)
    ///
    /// #Discussion
    ///
    /// For information on possible values, see MTLDataType.
    unsafe fn elementType(self) -> MTLDataType;

    /// The stride between array elements, in bytes. (read-only)
    unsafe fn stride(self) -> NSUInteger;

    /// When an array holds other arrays as its elements, returns
    /// an object that describes the underlying type.
    ///
    /// #Discussion
    ///
    /// Use this method if `elementType` is `MTLDataTypeArray`.
    unsafe fn elementArrayType(self) -> id;

    /// When an array holds structs as its elements, returns an
    /// object that describes the underlying type.
    ///
    /// #Discussion
    ///
    /// Use this method if `elementType` is `MTLDataTypeStruct`.
    unsafe fn elementStructType(self) -> id;
}

impl MTLArrayType for id {
    unsafe fn arrayLength(self) -> NSUInteger {
        msg_send![self, arrayLength]
    }
    
    unsafe fn elementType(self) -> MTLDataType {
        msg_send![self, elementType]
    }
    
    unsafe fn stride(self) -> NSUInteger {
        msg_send![self, stride]
    }
    
    unsafe fn elementArrayType(self) -> id {
        msg_send![self, elementArrayType]
    }
    
    unsafe fn elementStructType(self) -> id {
        msg_send![self, elementStructType]
    }
}
