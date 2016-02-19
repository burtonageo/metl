use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use objc::runtime::BOOL;
use MTLTextureType;

/// A `MTLArgument` object describes a single argument to a Metal function. Your
/// app uses the `MTLArgument` properties to read details about a function
/// argument as it was defined in the Metal Shading Language. You can determine
/// the argument’s data type, access restrictions, and its associated resource
/// type. For buffer, texture, and threadgroup memory arguments, additional
/// properties can be read to determine more details about the argument.
///
/// Your app does not create a `MTLArgument` object directly. Creating a
/// MTLRenderPipelineState or `MTLComputePipelineState` object can generate a
/// reflection object (`MTLRenderPipelineReflection` or
/// `MTLComputePipelineReflection`) that contains `MTLArgument` objects.
pub trait MTLArgument {
    /// The name of the function argument. (read-only)
    unsafe fn name(self) -> id;

    /// The function’s read and/or write access to the argument. (read-only)
    unsafe fn access(self) -> MTLArgumentAccess;

    /// If `YES`, this argument is used in a Metal shading language function and
    /// has been set to an index in an argument table. If `NO`, this argument is
    /// not used in a Metal shading language function and can be ignored.
    /// (read-only)
    unsafe fn active(self) -> BOOL;

    /// The index in the argument table that corresponds to the function argument. (read-only)
    ///
    /// #Discussion
    ///
    /// A command encoder (`MTLComputeCommandEncoder` or `MTLRenderCommandEncoder`) specifies the
    /// index in the corresponding argument table. For example, an app can call the
    /// `setTexture:atIndex:` method of `MTLComputeCommandEncoder` to specify an index in the
    /// texture argument table for a `MTLTexture` object that is used as an argument of a
    /// compute function.
    unsafe fn index(self) -> NSUInteger;

    /// The argument’s resource type. (read-only)
    ///
    /// #Discussion
    ///
    /// This property indicates which type of resource is used (buffer, texture, sampler, or
    /// threadgroup memory) in the shading language code. For information on possible values,
    /// see `MTLArgumentType`.
    unsafe fn getType(self) -> MTLArgumentType;

    /// The required byte alignment in memory for the buffer data in the function argument.
    /// (read-only)
    ///
    /// #Discussion
    ///
    /// If the argument is not a buffer, querying this property is a fatal error.
    unsafe fn bufferAlignment(self) -> NSUInteger;

    /// The size, in bytes, of the buffer data in the function argument. (read-only)
    ///
    /// #Discussion
    ///
    /// If the argument is not a buffer, querying this property is a fatal error.
    unsafe fn bufferDataSize(self) -> NSUInteger;

    /// The data type of the buffer data in the function argument. (read-only)
    ///
    /// #Discussion
    ///
    /// If the argument is not a buffer, querying this property is a fatal error.
    unsafe fn bufferDataType(self) -> MTLDataType;

    /// Returns a description of the structure data contained in the buffer argument.
    /// (read-only)
    ///
    /// #Discussion
    ///
    /// For information on possible returned values, see `MTLStructType`. If the
    /// argument is not a buffer, querying this property is a fatal error. If
    /// the argument is a buffer but the buffer’s data type is not a struct,
    /// the value of this property is nil.
    unsafe fn bufferStructType(self) -> id;

    /// The data type of a texture argument. (read-only)
    ///
    /// #Discussion
    ///
    /// For information on possible values, see `MTLDataType`. If the argument
    /// is not a texture, querying this property is a fatal error.
    unsafe fn textureDataType(self) -> MTLDataType;

    /// The texture type of a texture argument. (read-only)
    ///
    /// #Discussion
    ///
    /// For information on possible values, see `MTLTextureType`. If the
    /// argument is not a texture, querying this property is a fatal error.
    unsafe fn textureType(self) -> MTLTextureType;

    /// The required byte alignment in memory for the threadgroup data in
    /// the argument of a compute function. (read-only)
    ///
    /// #Discussion
    ///
    /// If the argument is not a threadgroup, querying this property is a
    /// fatal error. The Metal implementation determines this value.
    unsafe fn threadgroupMemoryAlignment(self) -> NSUInteger;

    /// The size, in bytes, of the threadgroup data in the argument of a
    /// compute function. (read-only)
    ///
    /// #Discussion
    ///
    /// If the argument is not a threadgroup, querying this property is
    /// a fatal error. The Metal implementation determines this value.
    unsafe fn threadgroupMemoryDataSize(self) -> NSUInteger;
}

impl MTLArgument for id {
    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn access(self) -> MTLArgumentAccess {
        msg_send![self, access]
    }

    unsafe fn active(self) -> BOOL {
        msg_send![self, access]
    }

    unsafe fn index(self) -> NSUInteger {
        msg_send![self, index]
    }

    unsafe fn getType(self) -> MTLArgumentType {
        msg_send![self, getType]
    }

    unsafe fn bufferAlignment(self) -> NSUInteger {
        msg_send![self, bufferAlignment]
    }
    unsafe fn bufferDataSize(self) -> NSUInteger {
        msg_send![self, bufferDataSize]
    }
    unsafe fn bufferDataType(self) -> MTLDataType {
        msg_send![self, bufferDataType]
    }
    unsafe fn bufferStructType(self) -> id {
        msg_send![self, bufferStructType]
    }

    unsafe fn textureDataType(self) -> MTLDataType {
        msg_send![self, textureDataType]
    }
    unsafe fn textureType(self) -> MTLTextureType {
        msg_send![self, textureType]
    }

    unsafe fn threadgroupMemoryAlignment(self) -> NSUInteger {
        msg_send![self, threadgroupMemoryAlignment]
    }
    unsafe fn threadgroupMemoryDataSize(self) -> NSUInteger {
        msg_send![self, threadGroupMemoryDataSize]
    }
}

/// Function access restrictions to argument data in the shading language code.
#[repr(usize)]
pub enum MTLArgumentAccess {
    MTLArgumentAccessReadOnly = 0,
    MTLArgumentAccessReadWrite = 1,
    MTLArgumentAccessWriteOnly = 2
}

/// The resource type associated with an argument of a function.
#[repr(usize)]
pub enum MTLArgumentType {
    MTLArgumentTypeBuffer = 0,
    MTLArgumentTypeThreadgroupMemory = 1,
    MTLArgumentTypeTexture = 2,
    MTLArgumentTypeSampler = 3
}

/// The data type of a function argument, as it is declared in the shading
/// language code.
#[repr(usize)]
pub enum MTLDataType {
    MTLDataTypeNone = 0,
    MTLDataTypeStruct = 1,
    MTLDataTypeArray = 2,

    MTLDataTypeFloat = 3,
    MTLDataTypeFloat2 = 4,
    MTLDataTypeFloat3 = 5,
    MTLDataTypeFloat4 = 6,

    MTLDataTypeFloat2x2 = 7,
    MTLDataTypeFloat2x3 = 8,
    MTLDataTypeFloat2x4 = 9,

    MTLDataTypeFloat3x2 = 10,
    MTLDataTypeFloat3x3 = 11,
    MTLDataTypeFloat3x4 = 12,
    MTLDataTypeFloat4x2 = 13,
    MTLDataTypeFloat4x3 = 14,
    MTLDataTypeFloat4x4 = 15,

    MTLDataTypeHalf = 16,
    MTLDataTypeHalf2 = 17,
    MTLDataTypeHalf3 = 18,
    MTLDataTypeHalf4 = 19,

    MTLDataTypeHalf2x2 = 20,
    MTLDataTypeHalf2x3 = 21,
    MTLDataTypeHalf2x4 = 22,

    MTLDataTypeHalf3x2 = 23,
    MTLDataTypeHalf3x3 = 24,
    MTLDataTypeHalf3x4 = 25,

    MTLDataTypeHalf4x2 = 26,
    MTLDataTypeHalf4x3 = 27,
    MTLDataTypeHalf4x4 = 28,

    MTLDataTypeInt = 29,
    MTLDataTypeInt2 = 30,
    MTLDataTypeInt3 = 31,
    MTLDataTypeInt4 = 32,
    MTLDataTypeUInt = 33,
    MTLDataTypeUInt2 = 34,
    MTLDataTypeUInt3 = 35,
    MTLDataTypeUInt4 = 36,

    MTLDataTypeShort = 37,
    MTLDataTypeShort2 = 38,
    MTLDataTypeShort3 = 39,
    MTLDataTypeShort4 = 40,

    MTLDataTypeUShort = 41,
    MTLDataTypeUShort2 = 42,
    MTLDataTypeUShort3 = 43,
    MTLDataTypeUShort4 = 44,

    MTLDataTypeChar = 45,
    MTLDataTypeChar2 = 46,
    MTLDataTypeChar3 = 47,
    MTLDataTypeChar4 = 48,

    MTLDataTypeUChar = 49,
    MTLDataTypeUChar2 = 50,
    MTLDataTypeUChar3 = 51,
    MTLDataTypeUChar4 = 52,

    MTLDataTypeBool = 53,
    MTLDataTypeBool2 = 54,
    MTLDataTypeBool3 = 55,
    MTLDataTypeBool4 = 56
}
