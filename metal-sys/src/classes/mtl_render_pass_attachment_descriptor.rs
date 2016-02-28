use cocoa::base::id;
use cocoa::foundation::NSUInteger;

/// A `MTLRenderPassAttachmentDescriptor` object is used to configure an
/// individual render target of a framebuffer. Each `MTLRenderPassAttachmentDescriptor`
/// object specifies one texture that a graphics rendering pass can write into.
///
/// Typically, you do not directly create `MTLRenderPassAttachmentDescriptor` objects.
/// Instead, when you initialize a `MTLRenderPassDescriptor` object, a default set of
/// attachment objects is created automatically. For each attachment that you intend to
/// use as a render target, you retrieve the `MTLRenderPassAttachmentDescriptor` object
/// from the render pass descriptor and configure its properties for how it is used
/// during this rendering pass.
///
/// You must set the attachment’s `texture` property, choosing an appropriate pixel
/// format. The `level`, `slice`, and `depthPlane` properties specify the mipmap level,
/// slice, and depth plane (for 3D textures) of the texture, respectively.
///
/// The `loadAction` and `storeAction` properties specify actions that are performed at
/// the start or end of a rendering pass, respectively, for the attachment. For example,
/// if the `loadAction` property of an attachment is set to `MTLLoadActionClear`, then
/// at the start of a rendering pass, the contents of the texture are filled with a
/// value that is appropriate for the type of attachment.
///
/// There are specific `MTLRenderPassAttachmentDescriptor` subclasses for color, depth,
/// and stencil attachments. Each subclass provides additional properties to configure
/// for that kind of attachment. Table 1 provides the list of subclasses.
///
/// ## Table 1 - Attachment descriptor subclasses
///
/// |=================================================================|
/// | Attachment type    | Render pass attachment descriptor subclass |
/// |=================================================================|
/// | Color attachment   | MTLRenderPassColorAttachmentDescriptor     |
/// |--------------------|--------------------------------------------|
/// | Depth attachment   | MTLRenderPassDepthAttachmentDescriptor     |
/// |--------------------|--------------------------------------------|
/// | Stencil attachment | MTLRenderPassStencilAttachmentDescriptor   |
/// |-----------------------------------------------------------------|
///
/// # Multisampling
///
/// To perform multisampled antialiased rendering, you use two textures. Attach a
/// `MTLTextureType2DMultisample` texture to the texture property and a 2D or cube texture
/// to the `resolveTexture` property. When a rendering command is executed, all rendering is
/// performed on the multisample texture as usual. Then, the contents of the multisample
/// texture are resolved and written into the resolve texture. The `resolveLevel`,
/// `resolveSlice`, and `resolveDepthPlane` properties specify where the resolved image
/// is written to. The contents of the multisample texture are then discarded.
pub trait MTLRenderPassAttachmentDescriptor {
    /// The texture object associated with this attachment.
    ///
    /// # Discussion
    ///
    /// You must set the attachment’s texture property, choosing an appropriate pixel format
    /// for the texture.
    ///
    ///    * To store color values in an attachment, use a texture with a color-renderable pixel
    ///      format.
    ///
    ///    * To store depth values, use a texture with a depth-renderable pixel format, such as
    ///      `MTLPixelFormatDepth32Float`.
    ///
    ///    * To store stencil values, use a texture with a stencil-renderable pixel format, such
    ///      as `MTLPixelFormatStencil8`.
    unsafe fn texture(self) -> id;
    unsafe fn setTexture(self, texture: id);

    /// The mipmap level of the texture used for rendering to the attachment.
    ///
    /// # Discussion
    ///
    /// The default value is `0`.
    unsafe fn level(self) -> NSUInteger;
    unsafe fn setLevel(self, level: NSUInteger);

    /// The slice of the texture used for rendering to the attachment.
    ///
    /// # Discussion
    ///
    /// The default value is `0`.
    unsafe fn slice(self) -> NSUInteger;
    unsafe fn setSlice(self, slice: NSUInteger);

    /// The depth plane of the texture used for rendering to the attachment.
    ///
    /// # Discussion
    ///
    /// If the texture is not a 3D texture, then this property is ignored.
    ///
    /// The default value is `0`.
    unsafe fn depthPlane(self) -> NSUInteger;
    unsafe fn setDepthPlane(self, depthPlane: NSUInteger);

    /// The action performed by this attachment at the start of a rendering pass
    /// for a render command encoder.
    ///
    /// # Discussion
    ///
    /// If your app renders all the pixels of the attachment for a given frame,
    /// use `MTLLoadActionDontCare`, which allows the GPU to avoid loading the
    /// existing contents of the texture. Otherwise, use `MTLLoadActionClear` to
    /// clear the previous contents of the attachment or `MTLLoadActionLoad` to
    /// preserve them. `MTLLoadActionClear` also avoids the cost of loading the
    /// existing texture contents, but it still incurs the cost of filling the
    /// destination with a solid color.
    ///
    /// The default value is `MTLLoadActionDontCare`.
    unsafe fn loadAction(self) -> MTLLoadAction;
    unsafe fn setLoadAction(self, loadAction: MTLLoadAction);

    /// The action performed by this attachment at the end of a rendering pass
    /// for a render command encoder.
    ///
    /// # Discussion
    ///
    /// If your app does not need the data in the texture after the rendering pass
    /// is complete, use `MTLStoreActionDontCare`. Otherwise, use
    /// `MTLStoreActionStore` if the texture is directly stored or
    /// `MTLStoreActionMultisampleResolve` if the texture is a multisampled texture.
    ///
    /// When storeAction is `MTLStoreActionMultisampleResolve`, the `resolveTexture`
    /// property must be set to the texture to use as the target for the resolve action.
    /// The `resolveLevel`, `resolveSlice`, and `resolveDepthPlane` properties may also
    /// be used for the multisample resolve operation to specify the mipmap level, cube
    /// slice, and depth plane of the multisample texture, respectively.
    ///
    /// The default value is `MTLStoreActionDontCare`.
    unsafe fn storeAction(self) -> MTLStoreAction;
    unsafe fn setStoreAction(self, storeAction: MTLStoreAction);

    /// The destination texture used when multisampled texture data is resolved
    /// into single sample values.
    ///
    /// # Discussion
    ///
    /// If the `storeAction` value is set to `MTLStoreActionMultisampleResolve`,
    /// then you must set this property to point to a texture. Otherwise, `resolveTexture`
    /// is ignored.
    unsafe fn resolveTexture(self) -> id;
    unsafe fn setResolveTexture(self, resolveTexture: id);

    /// The mipmap level of the texture used for the multisample resolve action.
    ///
    /// # Discussion
    ///
    /// If the value of storeAction is set to `MTLStoreActionMultisampleResolve`,
    /// this property is required.
    ///
    /// The default value is 0.
    unsafe fn resolveLevel(self) -> NSUInteger;
    unsafe fn setResolveLevel(self, resolveLevel: NSUInteger);

    /// The slice of the texture used for the multisample resolve action.
    ///
    /// # Discussion
    ///
    /// If the value of storeAction is set to `MTLStoreActionMultisampleResolve`,
    /// this property is required.
    ///
    /// The default value is 0.
    unsafe fn resolveSlice(self) -> NSUInteger;
    unsafe fn setResolveSlice(self, resolveSlice: NSUInteger);

    /// The depth plane of the texture used for the multisample resolve action.
    ///
    /// # Discussion
    ///
    /// If the value of storeAction is set to `MTLStoreActionMultisampleResolve`,
    /// this property is required.
    ///
    /// The default value is 0.
    unsafe fn resolveDepthPlane(self) -> NSUInteger;
    unsafe fn setResolveDepthPlane(self, resolveDepthPlane: NSUInteger);
}

/// The action performed at the start of a rendering pass for a render command encoder.
#[repr(usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLLoadAction {
    /// Each pixel in the attachment is allowed to take on any value at the start of the
    /// rendering pass.
    MTLLoadActionDontCare = 0,

    /// The existing contents of the texture are preserved.
    MTLLoadActionLoad = 1,

    /// A value is written to every pixel in the specified attachment.
    MTLLoadActionClear = 2
}

/// The action performed at the end of a rendering pass for a render command encoder.
#[repr(usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLStoreAction {
    /// After the rendering pass is complete, the attachment is left in an undefined
    /// state. This may improve performance because it enables the implementation to
    /// avoid any work necessary to save the rendering results.
    MTLStoreActionDontCare = 0,

    /// The final results of the rendering pass are saved into the attachment.
    MTLStoreActionStore = 1,

    /// The multisample values from the attachment are resolved into single sample values
    /// and stored in the texture specified by the `resolveTexture` property. The contents
    /// of the attachment are left undefined.
    MTLStoreActionMultisampleResolve = 2
}
