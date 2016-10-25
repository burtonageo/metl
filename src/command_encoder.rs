use cocoa::base::id;

pub struct CommandEncoder(id);

impl_from_into_raw!(CommandQueue, of protocol "MTLCommandEncoder");
