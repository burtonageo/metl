use cocoa::base::id;

pub trait MTLRenderCommandEncoder {

}

impl MTLRenderCommandEncoder for id {}

#[repr(c)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLCullMode {
    MTLCullModeNone,
    MTLCullModeFront,
    MTLCullModeBack
}

#[repr(c)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLDepthClipMode {
    MTLDepthClipModeClip,
    MTLDepthClipModeClamp
}

#[repr(c)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLWinding {
    MTLWindingClockwise,
    MTLWindingCounterClockwise
}
