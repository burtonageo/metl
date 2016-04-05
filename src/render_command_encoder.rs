use cocoa::base::id;
use {DepthStencilState, RenderPipelineState};

pub struct RenderCommandEncoder(id);

impl RenderCommandEncoder {
    pub fn set_blend_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unimplemented!();
    }

    pub fn set_cull_mode(&mut self, cull_mode: CullMode) {
        unimplemented!();
    }

    pub fn set_depth_bias_with_slope_and_clamp(&mut self, depth_bias: f32, slope: f32, clamp: f32) {
        unimplemented!();
    }

    pub fn set_depth_clip_mode(&mut self, depth_clip_mode: DepthClipMode) {
        unimplemented!();
    }

    pub fn set_depth_stencil_state(&mut self, depth_stencil_state: &DepthStencilState) {
        unimplemented!();
    }

    pub fn set_front_facing_winding(&mut self, winding: Winding) {
        unimplemented!();
    }
}

impl_from_into_raw!(RenderCommandEncoder, of protocol "MTLRenderCommandEncoder");

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CullMode {
    None,
    Front,
    Back
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DepthClipMode {
    Clip,
    Clamp
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Winding {
    Clockwise,
    CounterClockwise
}
