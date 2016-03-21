use cocoa::base::id;
use cocoa::foundation::NSUInteger;

/// A `MTLRenderPassColorAttachmentDescriptorArray` object contains an array of color
/// attachment descriptors.
pub trait MTLRenderPassColorAttachmentDescriptorArray {
    /// Returns the descriptor object for the specified color attachment.
    unsafe fn objectAtIndexedSubscript(self, attachmentIndex: NSUInteger) -> id;

    /// Sets the descriptor for the specified color attachment.
    ///
    /// # Description
    ///
    /// This method copies the color attachment information from the descriptor into
    /// the specified attachment in the array. Because the information is copied, the
    /// descriptor to can be modified and reused without affecting a previously set
    /// attachment.
    ///
    /// If this method is called with nil for attachment for any legal index, its color
    /// attachment descriptor is cleared to the default values.
    unsafe fn setObject_atIndexedSubscript(self, attachment: id, attachmentIndex: NSUInteger);
}

impl MTLRenderPassColorAttachmentDescriptorArray for id {
    unsafe fn objectAtIndexedSubscript(self, attachmentIndex: NSUInteger) -> id {
        msg_send![self, objectAtIndexedSubscript:attachmentIndex]
    }

    unsafe fn setObject_atIndexedSubscript(self, attachment: id, attachmentIndex: NSUInteger) {
        msg_send![self, setObject:attachment atIndexedSubscript:attachmentIndex]
    }
}
