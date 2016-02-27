use cocoa::base::id;

pub struct BlitCommandEncoder(id);

impl_from_into_raw!(BlitCommandEncoder, of protocol "MTLBlitCommandEncoder");

