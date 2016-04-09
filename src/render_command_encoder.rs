use cocoa::base::id;
use sys::{MTLCullMode, MTLDepthClipMode, MTLIndexType, MTLPrimitiveType, MTLTriangleFillMode,
          MTLVisibilityResultMode, MTLWinding};
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
