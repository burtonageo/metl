use cocoa::base::id;

/// The `MTLSamplerState` protocol defines the interface for a lightweight
/// object used to encode how a shader or compute kernel should sample a
/// texture. To create a sampler state object:
///
/// 1. Create a `MTLSamplerDescriptor` object.
///
/// 2. Set the desired properties of the sampler descriptor, including
///    filtering options, addressing modes, maximum anisotropy, and
///    level-of-detail parameters.
///
/// 3. Call the `newSamplerWithDescriptor:` method of the `MTLDevice` object,
///    which creates a sampler state object.
///
/// (Your app does not define a class that implements the `MTLSamplerState` protocol.)
///
/// You can either release the `MTLSamplerDescriptor` object or modify its property
/// values and reuse it to create more MTLSamplerState objects. The descriptor's
/// properties are only used during object creation; once created the behavior of
/// a sampler state object is fixed and cannot be changed.
pub trait MTLSamplerState {
    /// The device this sampler state was created from. (read-only)
    ///
    /// # Discussion
    ///
    /// This sampler can only be used with this device
    unsafe fn device(self) -> id;

    /// A string to help identify this sampler state object. (read-only)
    unsafe fn label(self) -> id;
}

impl MTLSamplerState for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }
}
