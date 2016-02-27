use cocoa::base::id;

pub struct ParallelRenderCommandEncoder(id);

impl_from_into_raw!(ParallelRenderCommandEncoder, of protocol "MTLRenderCommandEncoder");
