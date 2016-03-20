use cocoa::base::id;
use MTLClearColor;

/// A `MTLRenderPassColorAttachmentDescriptor` object is used to configure an individual
/// render target whose texture has a color-renderable pixel format.
pub trait MTLRenderPassColorAttachmentDescriptor {
    /// The color to use when the color attachment is cleared.
    ///
    /// # Discussion
    ///
    /// If the `loadAction` property of the attachment is set to `MTLLoadActionClear`, then at the
    /// start of a rendering pass, the contents of the texture is filled with the value stored in
    /// the `clearColor` property. Otherwise, `clearColor` is ignored.
    ///
    /// The `clearColor` property represents a set of RGBA components. The default value is
    ///  (0.0, 0.0, 0.0, 1.0) (black). Use the `MTLClearColorMake` function to construct a
    /// `MTLClearColor` value.
    unsafe fn clearColor(self) -> MTLClearColor;

    unsafe fn copy(self) -> Self;
}

impl MTLRenderPassColorAttachmentDescriptor for id {
    unsafe fn clearColor(self) -> MTLClearColor {
        msg_send![self, clearColor]
    }

    unsafe fn copy(self) -> Self {
        msg_send![self, copy]
    }
}
