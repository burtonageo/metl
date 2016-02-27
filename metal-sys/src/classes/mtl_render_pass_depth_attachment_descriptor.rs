use cocoa::base::id;

pub trait MTLRenderPassDepthAttachmentDescriptor {
    /// The depth to use when the depth attachment is cleared.
    ///
    /// # Discussion
    ///
    /// The default value is 1.0.
    ///
    /// If the `loadAction` property of the attachment is set to
    /// `MTLLoadActionClear`, then at the start of a rendering pass,
    /// the contents of the texture are filled with the value stored
    /// in the `clearDepth` property. Otherwise, `clearDepth` is ignored.
    unsafe fn clearDepth(self) -> f64;
    unsafe fn setClearDepth(self, clearDepth: f64);

    /// The filter used for an MSAA depth resolve operation.
    ///
    /// # Discussion
    ///
    /// The default value is `MTLMultisampleDepthResolveFilterSample0`.
    unsafe fn depthResolveFilter(self) -> MTLMultisampleDepthResolveFilter;
    unsafe fn setDepthResolveFilter(self, resolveFilter: MTLMultisampleDepthResolveFilter);
    
    unsafe fn copy(self) -> Self;
}

impl MTLRenderPassDepthAttachmentDescriptor for id {
    unsafe fn clearDepth(self) -> f64 {
        msg_send![self, clearDepth]
    }

    unsafe fn setClearDepth(self, clearDepth: f64) {
        msg_send![self, setClearDepth:clearDepth]
    }

    unsafe fn depthResolveFilter(self) -> MTLMultisampleDepthResolveFilter {
        msg_send![self, depthResolveFilter]
    }

    unsafe fn setDepthResolveFilter(self, resolveFilter: MTLMultisampleDepthResolveFilter) {
        msg_send![self, setDepthResolveFilter:resolveFilter]
    }

    unsafe fn copy(self) -> Self {
        msg_send![self, copy]
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MTLMultisampleDepthResolveFilter {
    MTLMultisampleDepthResolveFilterSample0 = 0,
    MTLMultisampleDepthResolveFilterMin = 1,
    MTLMultisampleDepthResolveFilterMax = 2,
}
