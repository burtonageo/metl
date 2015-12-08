use cocoa::base::id;
use core_foundation::date::CFTimeInterval;

/// The `MTLDrawable` protocol defines the interface for an object that represents
/// a displayable resource that can be used as a destination for rendering commands.
pub trait MTLDrawable {
    /// Displays the displayable resource.
    ///
    /// # Discussion
    ///
    /// This method is usually invoked by the command buffer’s scheduled handler.
    ///
    /// # Availability
    ///
    /// Available in iOS 8.0 and later.
    unsafe fn present(self);

    /// Displays the displayable resource at a given host time.
    ///
    /// # Parameters
    ///
    /// `presentationTime` - Time to display the resource, in seconds.
    ///
    /// # Discussion
    ///
    /// This method is usually invoked by the command buffer’s scheduled handler.
    /// 
    /// # Availability
    ///
    /// Available in iOS 8.0 and later.
    unsafe fn presentAtTime(self, presentationTime: CFTimeInterval);
}

impl MTLDrawable for id {
    unsafe fn present(self) {
        msg_send![self, present]
    } 

    unsafe fn presentAtTime(self, presentationTime: CFTimeInterval) {
        msg_send![self, presentAtTime:presentationTime]
    }
}
