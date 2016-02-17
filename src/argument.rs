use cocoa::base::id;
use cocoa::foundation::NSString;
use metal_sys::{MTLArgument, MTLArgumentAccess, MTLArgumentType, MTLDataType};
use objc::runtime::YES;
use std::borrow::Cow;
use std::ffi::CStr;
use std::mem;

pub struct Argument(id);

impl Argument {
	pub fn name(&self) -> Cow<str> {
		unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_string_lossy() }
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

	pub fn buffer_alignment(&self) -> usize {
		unsafe { self.0.bufferAlignment() as usize }
	}

	pub fn buffer_data_size(&self) -> usize {
		unsafe { self.0.bufferDataSize() as usize }
	}

	pub fn buffer_struct_type(&self) -> ! {
		unimplemented!()
	}

	pub fn texture_data_type(&self) -> ! {
		unimplemented!()
	}

	pub fn threadgroup_memory_alignment(&self) -> usize {
		// TODO(George): Error checking
		unsafe { self.0.threadgroupMemoryAlignment() as usize }
	}

	pub fn threadgroup_memory_data_size(&self) -> usize {
		// TODO(George): Error checking
		unsafe { self.0.threadgroupMemoryDataSize() as usize }
	}
}

#[repr(usize)]
pub enum ArgumentAccess {
	ReadOnly,
	ReadWrite,
	WriteOnly
}

impl From<MTLArgumentAccess> for ArgumentAccess {
	fn from(argument_access: MTLArgumentAccess) -> Self {
		unsafe { mem::transmute(argument_access) }
	}
}

impl Into<MTLArgumentAccess> for ArgumentAccess {
	fn into(self) -> MTLArgumentAccess {
		unsafe { mem::transmute(self) }
	}
}

#[repr(usize)]
pub enum ArgumentType {
	Buffer,
	ThreadgroupMemory,
	Texture,
	Sampler
}

impl From<MTLArgumentType> for ArgumentType {
	fn from(argument_type: MTLArgumentType) -> Self {
		unsafe { mem::transmute(argument_type) }
	}
}

impl Into<MTLArgumentType> for ArgumentType {
	fn into(self) -> MTLArgumentType {
		unsafe { mem::transmute(self) }
	}
}

#[repr(usize)]
pub enum DataType {
	None,
	Struct,
	Array,

	Float,
	Float2,
	Float3,
	Float4,

	Float2x2,
	Float2x3,
	Float2x4,

	Float3x2,
	Float3x3,
	Float3x4,
	Float4x2,
	Float4x3,
	Float4x4,

	Half,
	Half2,
	Half3,
	Half4,

	Half2x2,
	Half2x3,
	Half2x4,

	Half3x2,
	Half3x3,
	Half3x4,

	Half4x2,
	Half4x3,
	Half4x4,

	Int,
	Int2,
	Int3,
	Int4,
	UInt,
	UInt2,
	UInt3,
	UInt4,

	Short,
	Short2,
	Short3,
	Short4,

	UShort,
	UShort2,
	UShort3,
	UShort4,

	Char,
	Char2,
	Char3,
	Char4,

	UChar,
	UChar2,
	UChar3,
	UChar4,

	Bool,
	Bool2,
	Bool3,
	Bool4,
}

impl From<MTLDataType> for DataType {
	fn from(data_type: MTLDataType) -> Self {
		unsafe { mem::transmute(data_type) }
	}
}

impl Into<MTLDataType> for DataType {
	fn into(self) -> MTLDataType {
		unsafe { mem::transmute(self) }
	}
}

