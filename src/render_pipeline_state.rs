use cocoa::base::id;

pub struct RenderPipelineState(id);

impl_from_into_raw!(RenderPipelineState, of protocol "MTLRenderPipelineState");

pub struct RenderPipelineDescriptor(id);

impl_from_into_raw!(RenderPipelineState, of class "MTLRenderPipelineDescriptor");
