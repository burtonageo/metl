use cocoa::base::id;

pub struct RenderCommandEncoder(id);

impl_from_into_raw!(RenderCommandEncoder, of protocol "MTLRenderCommandEncoder");
