use cocoa::base::id;

pub struct Buffer(id);

impl_from_into_raw!(Buffer, of protocol "MTLBuffer");
