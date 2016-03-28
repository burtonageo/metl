use cocoa::base::id;
use cocoa::foundation::NSString;
use metal_sys::{MTLArgument, MTLArgumentAccess, MTLArgumentType, MTLDataType};
use objc::runtime::YES;
use std::ffi::CStr;
use {FromRaw, StructType, TextureType};

pub struct Argument(id);

impl Argument {
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_str().unwrap_or(&"") }
    }

    pub fn access(&self) -> ArgumentAccess {
        unsafe { self.0.access().into() }
    }

    pub fn is_active(&self) -> bool {
        unsafe { self.0.active() == YES }
    }

    pub fn index(&self) -> usize {
        unsafe { self.0.index() as usize }
    }

    pub fn get_type(&self) -> ArgumentType {
        unsafe { self.0.getType().into() }
    }

    pub fn buffer_alignment(&self) -> Option<usize> {
        if self.get_type() == ArgumentType::Buffer {
            unsafe { Some(self.0.bufferAlignment() as usize) }
        } else {
            None
        }
    }

    pub fn buffer_data_size(&self) -> Option<usize> {
        if self.get_type() == ArgumentType::Buffer {
            unsafe { Some(self.0.bufferDataSize() as usize) }
        } else {
            None
        }
    }

    pub fn buffer_data_type(&self) -> Option<DataType> {
        if self.get_type() == ArgumentType::Buffer {
            unsafe { Some(self.0.bufferDataType().into()) }
        } else {
            None
        }
    }

    pub fn buffer_struct_type(&self) -> Option<StructType> {
        if self.get_type() == ArgumentType::Buffer {
            let struct_type = unsafe { self.0.bufferStructType() };
            StructType::from_raw(struct_type).ok()
        } else {
            None
        }
    }

    pub fn texture_data_type(&self) -> Option<DataType> {
        if self.get_type() == ArgumentType::Texture {
            unsafe { Some(self.0.textureDataType().into()) }
        } else {
            None
        }
    }

    pub fn texture_type(&self) -> Option<TextureType> {
        if self.get_type() == ArgumentType::Texture {
            unsafe { Some(self.0.textureType().into()) }
        } else {
            None
        }
    }

    pub fn threadgroup_memory_alignment(&self) -> Option<usize> {
        if self.get_type() == ArgumentType::ThreadgroupMemory {
            unsafe { Some(self.0.threadgroupMemoryAlignment() as usize) }
        } else {
            None
        }
    }

    pub fn threadgroup_memory_data_size(&self) -> Option<usize> {
        if self.get_type() == ArgumentType::ThreadgroupMemory {
            unsafe { Some(self.0.threadgroupMemoryDataSize() as usize) }
        } else {
            None
        }
    }
}

impl_from_into_raw!(Argument, of class "MTLArgument");

convertible_enum!{
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum ArgumentAccess : MTLArgumentAccess {
        ReadOnly => MTLArgumentAccessReadOnly,
        ReadWrite => MTLArgumentAccessReadWrite,
        WriteOnly => MTLArgumentAccessWriteOnly
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum ArgumentType : MTLArgumentType {
        Buffer => MTLArgumentTypeBuffer,
        ThreadgroupMemory => MTLArgumentTypeThreadgroupMemory,
        Texture => MTLArgumentTypeTexture,
        Sampler => MTLArgumentTypeSampler
    }
}


convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum DataType : MTLDataType {
        None => MTLDataTypeNone,
        Struct => MTLDataTypeStruct,
        Array => MTLDataTypeArray,

        Float => MTLDataTypeFloat,
        Float2 => MTLDataTypeFloat2,
        Float3 => MTLDataTypeFloat3,
        Float4 => MTLDataTypeFloat4,

        Float2x2 => MTLDataTypeFloat2x2,
        Float2x3 => MTLDataTypeFloat2x3,
        Float2x4 => MTLDataTypeFloat2x4,

        Float3x2 => MTLDataTypeFloat3x2,
        Float3x3 => MTLDataTypeFloat3x3,
        Float3x4 => MTLDataTypeFloat3x4,
        Float4x2 => MTLDataTypeFloat4x2,
        Float4x3 => MTLDataTypeFloat4x3,
        Float4x4 => MTLDataTypeFloat4x4,

        Half => MTLDataTypeHalf,
        Half2 => MTLDataTypeHalf2,
        Half3 => MTLDataTypeHalf3,
        Half4 => MTLDataTypeHalf4,

        Half2x2 => MTLDataTypeHalf2x2,
        Half2x3 => MTLDataTypeHalf2x3,
        Half2x4 => MTLDataTypeHalf2x4,

        Half3x2 => MTLDataTypeHalf3x2,
        Half3x3 => MTLDataTypeHalf3x3,
        Half3x4 => MTLDataTypeHalf3x4,

        Half4x2 => MTLDataTypeHalf4x2,
        Half4x3 => MTLDataTypeHalf4x3,
        Half4x4 => MTLDataTypeHalf4x4,

        Int => MTLDataTypeInt,
        Int2 => MTLDataTypeInt2,
        Int3 => MTLDataTypeInt3,
        Int4 => MTLDataTypeInt4,
        UInt => MTLDataTypeUInt,
        UInt2 => MTLDataTypeUInt2,
        UInt3 => MTLDataTypeUInt3,
        UInt4 => MTLDataTypeUInt4,

        Short => MTLDataTypeShort,
        Short2 => MTLDataTypeShort2,
        Short3 => MTLDataTypeShort3,
        Short4 => MTLDataTypeShort4,

        UShort => MTLDataTypeUShort,
        UShort2 => MTLDataTypeUShort2,
        UShort3 => MTLDataTypeUShort3,
        UShort4 => MTLDataTypeUShort4,

        Char => MTLDataTypeChar,
        Char2 => MTLDataTypeChar2,
        Char3 => MTLDataTypeChar3,
        Char4 => MTLDataTypeChar4,

        UChar => MTLDataTypeUChar,
        UChar2 => MTLDataTypeUChar2,
        UChar3 => MTLDataTypeUChar3,
        UChar4 => MTLDataTypeUChar4,

        Bool => MTLDataTypeBool,
        Bool2 => MTLDataTypeBool2,
        Bool3 => MTLDataTypeBool3,
        Bool4 => MTLDataTypeBool4
    }
}
