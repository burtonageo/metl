use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use sys::{MTLCullMode, MTLDepthClipMode, MTLIndexType, MTLPrimitiveType, MTLTriangleFillMode,
          MTLRenderCommandEncoder, MTLVisibilityResultMode, MTLWinding, MTLScissorRect,
          MTLViewport};
use {AsRaw, DepthStencilState, RenderPipelineState};

pub struct RenderCommandEncoder(id);

impl RenderCommandEncoder {
    pub fn set_blend_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { self.0.setBlendColorRed_green_blue_alpha(red, green, blue, alpha) }
    }

    pub fn set_cull_mode<C: Into<MTLCullMode>>(&mut self, cull_mode: C) {
        unsafe { self.0.setCullMode(cull_mode.into()) }
    }

    pub fn set_depth_bias_with_slope_and_clamp(&mut self, depth_bias: f32, slope: f32, clamp: f32) {
        unsafe { self.0.setDepthBias_slopeScale_clamp(depth_bias, slope, clamp) }
    }

    pub fn set_depth_clip_mode<DC: Into<MTLDepthClipMode>>(&mut self, depth_clip_mode: DC) {
        unsafe { self.0.setDepthClipMode(depth_clip_mode.into()) }
    }

    pub fn set_depth_stencil_state(&mut self, depth_stencil_state: &DepthStencilState) {
        unsafe { self.0.setDepthStencilState(*depth_stencil_state.as_raw()) }
    }

    pub fn set_front_facing_winding<W: Into<MTLWinding>>(&mut self, winding: W) {
        unsafe { self.0.setFrontFacingWinding(winding.into()) }
    }

    pub fn set_render_pipeline_state(&mut self, render_pipeline_state: &RenderPipelineState) {
        unsafe { self.0.setRenderPipelineState(*render_pipeline_state.as_raw()) }
    }

    pub fn set_scissor_rect<R: Into<MTLScissorRect>>(&mut self, scissor_rect: R) {
        unsafe { self.0.setScissorRect(scissor_rect.into()) }
    }

    pub fn set_stencil_reference_value_front_and_back(&mut self, front_reference_value: u32,
                                                      back_reference_value: u32) {
        unsafe { self.0.setStencilFrontReferenceValue_backReferenceValue(front_reference_value,
                                                                         back_reference_value) }
    }

    pub fn set_stencil_reference_values(&mut self, reference_values: u32) {
        unsafe { self.0.setStencilReferenceValue(reference_values) }
    }

    pub fn set_triangle_fill_mode<T: Into<MTLTriangleFillMode>>(&mut self, triangle_fill_mode: T) {
        unsafe { self.0.setTriangleFillMode(triangle_fill_mode.into()) }
    }

    pub fn set_viewport<V: Into<MTLViewport>>(&mut self, viewport: V) {
        unsafe { self.0.setViewport(viewport.into()) }
    }

    pub fn set_visibility_result_mode_with_offset<V: Into<MTLVisibilityResultMode>>(&mut self,
                                                                                    visibility_result_mode: V,
                                                                                    offset: usize) {
        unsafe { self.0.setVisibilityResultMode_offset(visibility_result_mode.into(), offset as NSUInteger) }
    }
}

impl_from_into_raw!(RenderCommandEncoder, of protocol "MTLRenderCommandEncoder");

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum CullMode: MTLCullMode {
        None => MTLCullModeNone,
        Front => MTLCullModeFront,
        Back => MTLCullModeBack
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum DepthClipMode: MTLDepthClipMode {
        Clip => MTLDepthClipModeClip,
        Clamp => MTLDepthClipModeClamp
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Winding: MTLWinding {
        Clockwise => MTLWindingClockwise,
        CounterClockwise => MTLWindingCounterClockwise
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum PrimitiveType: MTLPrimitiveType {
        Point => MTLPrimitiveTypePoint,
        Line => MTLPrimitiveTypeLine,
        LineStrip => MTLPrimitiveTypeLineStrip,
        Triangle => MTLPrimitiveTypeTriangle,
        TriangleStrip => MTLPrimitiveTypeTriangleStrip
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum IndexType: MTLIndexType {
        UInt16 => MTLIndexTypeUInt16,
        UInt32 => MTLIndexTypeUInt32
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum VisibilityResultMode: MTLVisibilityResultMode {
        Disabled => MTLVisibilityResultModeDisabled,
        Boolean => MTLVisibilityResultModeBoolean,
        Counting => MTLVisibilityResultModeCounting
    }
}

convertible_enum! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum TriangleFillModeFill: MTLTriangleFillMode {
        Fill => MTLTriangleFillModeFill,
        Lines => MTLTriangleFillModeLines
    }
}
