use cocoa::base::id;

pub struct StructType(id);

impl StructType {
	pub fn members(&self) -> ! {
		unimplemented!()
	}

	pub fn member_by_name(&self, name: &str) -> ! {
		unimplemented!()
	}
}

impl_from_into_raw!(StructType, of class "MTLStructType");
