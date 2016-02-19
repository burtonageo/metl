use cocoa::base::id;
use sys::MTLArrayType;
use {DataType, FromRaw, StructType};

pub struct ArrayType(id);

impl ArrayType {
    pub fn array_length(&self) -> usize {
        unsafe { self.0.arrayLength() as usize }
    }

    pub fn element_type(&self) -> DataType {
        unsafe { self.0.elementType().into() }
    }

    pub fn stride(&self) -> usize {
        unsafe { self.0.stride() as usize() }
    }

    pub fn element_array_type(&self) -> Option<Self> {
        unsafe { Self::from_raw(self.0.elementArrayType()).ok() }
    }

    pub fn element_struct_type(&self) -> Option<StructType> {
        unsafe { StructType::from_raw(self.0.elementStructType()).ok() }
    }
}

impl_from_into_raw!(ArrayType, of class "MTLArrayType");
