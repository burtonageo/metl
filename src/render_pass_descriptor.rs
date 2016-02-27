use cocoa::base::id;

pub struct RenderPassDescriptor(id);

impl_from_into_raw!(RenderPassDescriptor, of class "MTLRenderPassDescriptor");
