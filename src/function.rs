use cocoa::base::id;

pub struct Function(id);

impl_from_into_raw!(Function, "MTLFunction");
