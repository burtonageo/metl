use cocoa::base::{id, nil};
use {DataType, StructType};

struct ArrayType(id);

impl ArrayType {
	fn arrayLength(self) -> usize {
		unsafe { self.0.arrayLength() as usize }
	}

	fn elementType(self) -> DataType {
		unsafe { self.0.elementType().into() }
	}

	fn stride(self) -> usize {
		unsafe { self.0.stride() as usize() }
	}

	fn elementArrayType(self) -> Option<Self> {
		let array_type = unsafe { self.0.elementArrayType() }
		if array_type != nil { Some(ArrayType(array_type)) } else { None }
	}

	fn elementStructType(self) -> Option<StructType> {
		let struct_type = unsafe { self.0.elementStructType() }
		if struct_type != nil { Some(StructType(struct_type)) } else { None }
	}
}

impl_from_into_raw!(ArrayType, of class "MTLArrayType");
