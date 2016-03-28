use cocoa::base::id;
use cocoa::foundation::NSString;
use std::ffi::CStr;
use sys::MTLStructMember;
use {ArrayType, DataType, FromRaw, StructType};

pub struct StructMember(id);

impl StructMember {
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.name().UTF8String()).to_str().unwrap_or(&"") }
    }

    pub fn data_type(&self) -> DataType {
        unsafe { self.0.dataType().into() }
    }

    pub fn offset(&self) -> usize {
        unsafe { MTLStructMember::offset(self.0) as usize }
    }

    pub fn array_type(&self) -> Option<ArrayType> {
        unsafe { ArrayType::from_raw(self.0.arrayType()).ok() }
    }

    pub fn struct_type(&self) -> Option<StructType> {
        unsafe { StructType::from_raw(self.0.structType()).ok() }
    }
}

impl_from_into_raw!(StructMember, of class "MTLStructMember");
