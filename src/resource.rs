use cocoa::base::id;

pub struct Resource(id);

impl_from_into_raw!(Resource, of protocol "MTLResource");
