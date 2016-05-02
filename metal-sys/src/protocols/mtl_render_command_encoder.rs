use cocoa::base::id;
use cocoa::foundation::{NSInteger, NSRange, NSUInteger};
use libc::{uint32_t, c_void};
use types::{MTLScissorRect, MTLViewport};

pub trait MTLRenderCommandEncoder {
    unsafe fn setBlendColorRed_green_blue_alpha(self, red: f32, green: f32, blue: f32, alpha: f32);
    unsafe fn setCullMode(self, cullMode: MTLCullMode);
    unsafe fn setDepthBias_slopeScale_clamp(self, depthBias: f32, slopeScale: f32, clamp: f32);
    unsafe fn setDepthClipMode(self, depthClipMode: MTLDepthClipMode);
    unsafe fn setDepthStencilState(self, depthStencilState: id);
    unsafe fn setFrontFacingWinding(self, frontFacingWinding: MTLWinding);
    unsafe fn setRenderPipelineState(self, renderPipelineState: id);
    unsafe fn setScissorRect(self, scissorRect: MTLScissorRect);
    unsafe fn setStencilFrontReferenceValue_backReferenceValue(self, frontReferenceValue: uint32_t,
                                                               backReferenceValue: uint32_t);
    unsafe fn setStencilReferenceValue(self, referenceValue: uint32_t);
    unsafe fn setTriangleFillMode(self, fillMode: MTLTriangleFillMode);
    unsafe fn setViewport(self, viewport: MTLViewport);
    unsafe fn setVisibilityResultMode_offset(self, visibilityResultMode: MTLVisibilityResultMode,
                                             offset: NSUInteger);

    unsafe fn setVertexBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger);
    unsafe fn setVertexBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange);
    unsafe fn setVertexBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger);
    unsafe fn setVertexBytes_length_atIndex(self, bytes: *const c_void, length: NSUInteger, index: NSUInteger);
    unsafe fn setVertexSamplerState_atIndex(self, sampler: id, index: NSUInteger);
    unsafe fn setVertexSamplerStates_withRange(self, samplers: *const id, range: NSRange);
    unsafe fn setVertexSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, samplers: id, lodMinClamp: f32,
                                                                    lodMaxClamp: f32, index: NSUInteger);
    unsafe fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id,
                                                                         lodMinClamps: *const f32,
                                                                         lodMaxClamps: *const f32,
                                                                         range: NSRange);
    unsafe fn setVertexTexture_atIndex(self, texture: id, index: NSUInteger);
    unsafe fn setVertexTextures_withRange(self, textures: *const id, range: NSRange);

    unsafe fn setFragmentBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger);
    unsafe fn setFragmentBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger,
                                                   range: NSRange);
    unsafe fn setFragmentBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger);
    unsafe fn setFragmentBytes_length_atIndex(self, bytes: *const c_void, length: NSUInteger, index: NSUInteger);
    unsafe fn setFragmentSamplerState_atIndex(self, sampler: id, index: NSUInteger);
    unsafe fn setFragmentSamplerStates_withRange(self, samplers: *const id, range: NSRange);
    unsafe fn setFragmentSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, samplers: id, lodMinClamp: f32,
                                                                      lodMaxClamp: f32, index: NSUInteger);
    unsafe fn setFragmentSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id,
                                                                           lodMinClamps: f32,
                                                                           lodMaxClamps: f32,
                                                                           range: NSRange);
    unsafe fn setFragmentTexture_atIndex(self, texture: id, index: NSUInteger);
    unsafe fn setFragmentTextures_withRange(self, textures: *const id, range: NSRange);

    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(self,
        primitiveType: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger,
        instanceCount: NSUInteger, baseInstance: NSUInteger);
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount(self, primitiveType: MTLPrimitiveType,
                                                                   vertexStart: NSUInteger, vertexCount: NSUInteger,
                                                                   instanceCount: NSUInteger);
    unsafe fn drawPrimitives_vertexStart_vertexCount(self, vertexStart: NSUInteger, vertexCount: NSUInteger);
    unsafe fn drawPrimitives_indirectBuffer_indirectBufferOffset(self, primitiveType: MTLPrimitiveType,
                                                                 indirectBuffer: id, indirectBufferOffset: NSUInteger);
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance(
        self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id,
        indexBufferOffset: NSUInteger, instanceCount: NSUInteger, baseVertex: NSInteger, baseInstance: NSUInteger);
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount(
        self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id,
        indexBufferOffset: NSUInteger, instanceCount: NSUInteger);
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset(
        self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id,
        indexBufferOffset: NSUInteger);
    unsafe fn drawIndexedPrimitives_indexType_indexBuffer_indexBufferOffset_indirectBuffer_indirectBufferOffset(self,
        primitiveType: MTLPrimitiveType, indexType: MTLIndexType, indexBuffer: id, indexBufferOffset: NSUInteger,
        indirectBuffer: id, indirectBufferOffset: NSUInteger);
}

impl MTLRenderCommandEncoder for id {
    unsafe fn setBlendColorRed_green_blue_alpha(self, red: f32, green: f32, blue: f32, alpha: f32) {
        unimplemented!();
    }
    unsafe fn setCullMode(self, cullMode: MTLCullMode) {
        unimplemented!();
    }
    unsafe fn setDepthBias_slopeScale_clamp(self, depthBias: f32, slopeScale: f32, clamp: f32) {
        unimplemented!();
    }
    unsafe fn setDepthClipMode(self, depthClipMode: MTLDepthClipMode) {
        unimplemented!();
    }
    unsafe fn setDepthStencilState(self, depthStencilState: id) {
        unimplemented!();
    }
    unsafe fn setFrontFacingWinding(self, frontFacingWinding: MTLWinding) {
        unimplemented!();
    }
    unsafe fn setRenderPipelineState(self, renderPipelineState: id) {
        unimplemented!();
    }
    unsafe fn setScissorRect(self, scissorRect: MTLScissorRect) {
        unimplemented!();
    }
    unsafe fn setStencilFrontReferenceValue_backReferenceValue(self, frontReferenceValue: uint32_t,
                                                               backReferenceValue: uint32_t) {
        unimplemented!();
    }
    unsafe fn setStencilReferenceValue(self, referenceValue: uint32_t) {
        unimplemented!();
    }
    unsafe fn setTriangleFillMode(self, fillMode: MTLTriangleFillMode) {
        unimplemented!();
    }
    unsafe fn setViewport(self, viewport: MTLViewport) {
        unimplemented!();
    }
    unsafe fn setVisibilityResultMode_offset(self, visibilityResultMode: MTLVisibilityResultMode,
                                             offset: NSUInteger) {
        unimplemented!();
    }
    
    unsafe fn setVertexBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setVertexBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange) {
        unimplemented!();
    }
    unsafe fn setVertexBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setVertexBytes_length_atIndex(self, bytes: *const c_void, length: NSUInteger, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setVertexSamplerState_atIndex(self, sampler: id, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setVertexSamplerStates_withRange(self, samplers: *const id, range: NSRange) {
        unimplemented!();
    }
    unsafe fn setVertexSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, samplers: id, lodMinClamp: f32,
                                                                    lodMaxClamp: f32, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id,
                                                                         lodMinClamps: *const f32,
                                                                         lodMaxClamps: *const f32,
                                                                         range: NSRange) {
        unimplemented!();
    }
    unsafe fn setVertexTexture_atIndex(self, texture: id, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setVertexTextures_withRange(self, textures: *const id, range: NSRange) {
        unimplemented!();
    }
    
    unsafe fn setFragmentBuffer_offset_atIndex(self, buffer: id, offset: NSUInteger, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setFragmentBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger,
                                                   range: NSRange) {
        unimplemented!();
    }
    unsafe fn setFragmentBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setFragmentBytes_length_atIndex(self, bytes: *const c_void, length: NSUInteger, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setFragmentSamplerState_atIndex(self, sampler: id, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setFragmentSamplerStates_withRange(self, samplers: *const id, range: NSRange) {
        unimplemented!();
    }
    unsafe fn setFragmentSamplerState_lodMinClamp_lodMaxClamp_atIndex(self, samplers: id, lodMinClamp: f32,
                                                                      lodMaxClamp: f32, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setFragmentSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id,
                                                                           lodMinClamps: f32,
                                                                           lodMaxClamps: f32,
                                                                           range: NSRange) {
        unimplemented!();
    }
    unsafe fn setFragmentTexture_atIndex(self, texture: id, index: NSUInteger) {
        unimplemented!();
    }
    unsafe fn setFragmentTextures_withRange(self, textures: *const id, range: NSRange) {
        unimplemented!();
    }
    
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(self,
        primitiveType: MTLPrimitiveType, vertexStart: NSUInteger, vertexCount: NSUInteger,
        instanceCount: NSUInteger, baseInstance: NSUInteger) {
        unimplemented!();
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount(self, primitiveType: MTLPrimitiveType,
                                                                   vertexStart: NSUInteger, vertexCount: NSUInteger,
                                                                   instanceCount: NSUInteger) {
        unimplemented!();
    }
    unsafe fn drawPrimitives_vertexStart_vertexCount(self, vertexStart: NSUInteger, vertexCount: NSUInteger) {
        unimplemented!();
    }
    unsafe fn drawPrimitives_indirectBuffer_indirectBufferOffset(self, primitiveType: MTLPrimitiveType,
                                                                 indirectBuffer: id, indirectBufferOffset: NSUInteger) {
        unimplemented!();
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance(
        self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id,
        indexBufferOffset: NSUInteger, instanceCount: NSUInteger, baseVertex: NSInteger, baseInstance: NSUInteger) {
        unimplemented!();
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount(
        self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id,
        indexBufferOffset: NSUInteger, instanceCount: NSUInteger) {
        unimplemented!();
    }
    unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset(
        self, primitiveType: MTLPrimitiveType, indexCount: NSUInteger, indexType: MTLIndexType, indexBuffer: id,
        indexBufferOffset: NSUInteger) {
        unimplemented!();
    }
    unsafe fn drawIndexedPrimitives_indexType_indexBuffer_indexBufferOffset_indirectBuffer_indirectBufferOffset(self,
        primitiveType: MTLPrimitiveType, indexType: MTLIndexType, indexBuffer: id, indexBufferOffset: NSUInteger,
        indirectBuffer: id, indirectBufferOffset: NSUInteger) {
        unimplemented!();
    }
}

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
pub enum MTLTriangleFillMode {
    MTLTriangleFillModeFill = 0,
    MTLTriangleFillModeLines = 1
}
