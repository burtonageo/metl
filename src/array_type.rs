use cocoa::base::{id, nil};
use {DataType, StructType};

struct ArrayType(id);

impl ArrayType {
	pub fn array_length(self) -> usize {
		unsafe { self.0.arrayLength() as usize }
	}

	pub fn element_type(self) -> DataType {
		unsafe { self.0.elementType().into() }
	}

	pub fn stride(self) -> usize {
		unsafe { self.0.stride() as usize() }
	}

	pub fn element_array_type(self) -> Option<Self> {
		let array_type = unsafe { self.0.elementArrayType() }
		if array_type != nil { Some(ArrayType(array_type)) } else { None }
	}

	pub fn element_struct_type(self) -> Option<StructType> {
		let struct_type = unsafe { self.0.elementStructType() }
		if struct_type != nil { Some(StructType(struct_type)) } else { None }
	}
}

impl_from_into_raw!(ArrayType, of class "MTLArrayType");
