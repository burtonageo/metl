use cocoa::base::id;

pub trait MTLRenderCommandEncoder {

}

impl MTLRenderCommandEncoder for id {}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLPrimitiveType {
    MTLPrimitiveTypePoint = 0,
    MTLPrimitiveTypeLine = 1,
    MTLPrimitiveTypeLineStrip = 2,
    MTLPrimitiveTypeTriangle = 3,
    MTLPrimitiveTypeTriangleStrip = 4
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLIndexType {
    MTLIndexTypeUInt16 = 0,
    MTLIndexTypeUInt32 = 1
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLVisibilityResultMode {
    MTLVisibilityResultModeDisabled = 0,
    MTLVisibilityResultModeBoolean = 1,
    MTLVisibilityResultModeCounting = 2
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLCullMode {
    MTLCullModeNone = 0,
    MTLCullModeFront = 1,
    MTLCullModeBack = 2
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLDepthClipMode {
    MTLDepthClipModeClip = 0,
    MTLDepthClipModeClamp = 1
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLWinding {
    MTLWindingClockwise = 0,
    MTLWindingCounterClockwise = 1
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLTriangleFillModeFill {
    MTLTriangleFillModeFill = 0,
    MTLTriangleFillModeLines = 1
}
