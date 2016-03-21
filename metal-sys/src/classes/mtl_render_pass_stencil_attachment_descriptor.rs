use cocoa::base::id;
use libc::uint32_t;

/// A `MTLRenderPassStencilAttachmentDescriptor` object is used to configure
/// an individual render target that has a texture with a stencil-renderable
/// pixel format.
pub trait MTLRenderPassStencilAttachmentDescriptor {
    /// The stencil value to use when the stencil attachment is cleared.
    ///
    /// # Discussion
    ///
    /// If the `loadAction` property of an attachment is set to `MTLLoadActionClear`,
    /// then at the start of a rendering pass, the contents of the texture is filled
    /// with the value stored in the `clearStencil` property. Otherwise, `clearStencil`
    /// is ignored.
    ///
    /// The default value is `0`.
    unsafe fn clearStencil(self) -> uint32_t;
    unsafe fn setClearStencil(self, stencil: uint32_t);
}

impl MTLRenderPassStencilAttachmentDescriptor for id {
    unsafe fn clearStencil(self) -> uint32_t {
        msg_send![self, clearStencil]
    }

    unsafe fn setClearStencil(self, stencil: uint32_t) {
        msg_send![self, setClearStencil:stencil]
    }
}
