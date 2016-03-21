use cocoa::base::id;

/// The `MTLResource` protocol defines the interface for any resource
/// object that represents an allocation of GPU memory.
///
/// Your app does not define classes that implement this protocol. Instead,
/// your app allocates buffer objects (`MTLBuffer`) or texture objects (`MTLTexture`)
/// that conform to this protocol.
pub trait MTLResource {
    /// The CPU cache mode used for the CPU mapping of the resource. (read-only).
    ///
    /// # Discussion
    ///
    /// The cache mode is set when you first create a resource and cannot be changed
    /// afterwards.
    ///
    /// Write-combined memory can have surprising performance pitfalls. Consider using
    /// the write-combined cache mode only if writing to normally cached buffers is known
    /// to cause performance issues due to cache pollution. For more information, see the
    /// CPU Cache Modes constants section.
    unsafe fn cpuCacheMode(self) -> MTLCPUCacheMode;

    /// The storage mode used for the CPU mapping of the resource. (read-only).
    ///
    /// # Discussion
    ///
    /// The storage mode is set when you first create a resource and cannot be
    /// changed afterward.
    unsafe fn storageMode(self) -> MTLStorageMode;

    /// The device that created this resource. (read-only).
    ///
    /// # Discussion
    ///
    /// A resource is always associated with the `MTLDevice` that created it and
    /// can be used only with that device.
    unsafe fn device(self) -> id;

    /// A string used to identify this resource.
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, label: id);

    /// Sets whether a resource can be purged synchronously.
    ///
    /// # Discussion
    ///
    /// If state is `MTLPurgeableStateKeepCurrent`, the current purgeable state is
    /// queried and is not changed.
    ///
    /// If state is `MTLPurgeableStateNonVolatile`, the resource is marked to inform the
    ///caller that the data should not be discarded.
    ///
    /// If state is `MTLPurgeableStateEmpty`, the resource is marked as data that can be
    /// discarded, because the caller no longer needs the contents of the resource.
    ///
    /// If state is `MTLPurgeableStateVolatile`, the resource is marked as data that can
    /// be discarded, even if the caller may need the resource. `MTLResource` objects can
    /// be made purgeable, even if the caller may need the resource, where the implementation
    /// can reclaim the underlying storage at any time without informing the app. Purgeable
    /// resources may enable an app to keep larger caches of idle memory that may be useful
    /// again in the future without the risk of preventing the allocation of more important memory.
    ///
    /// When you use purgeable memory, make sure the block of memory is locked before you access
    /// it. This locking mechanism is necessary to ensure that auto-removal policies do not discard
    /// the data while you are accessing it. Similarly, the locking mechanism ensures that the
    /// virtual memory system has not already discarded the data.
    unsafe fn setPurgeableState(self, state: MTLPurgeableState) -> MTLPurgeableState;
}

impl MTLResource for id {
    unsafe fn cpuCacheMode(self) -> MTLCPUCacheMode {
        msg_send![self, cpuCacheMode]
    }

    unsafe fn storageMode(self) -> MTLStorageMode {
        msg_send![self, storageMode]
    }

    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn label(self) -> id {
        msg_send![self, label]
    }

    unsafe fn setLabel(self, label: id) {
        msg_send![self, setLabel:label]
    }

    unsafe fn setPurgeableState(self, state: MTLPurgeableState) -> MTLPurgeableState {
        msg_send![self, setPurgeableState:state]
    }
}

/// The CPU cache mode used for the CPU mapping of a resource.
///
/// # Discussion
///
/// Write-combined memory is optimized for resources that the CPU writes into,
/// but never reads. On some implementations, writes may bypass caches, which
/// avoids cache pollution. Read actions may perform very poorly.
///
/// Applications should investigate changing the cache mode only if writing to
/// normally cached buffers is known to cause performance issues due to cache
/// pollution, as write-combined memory can have surprising performance pitfalls.
/// Another approach is to use nontemporal writes to normally cached memory (STNP
/// on ARMv8, _mm_stream_* on x86_64).
#[repr(usize)]
pub enum MTLCPUCacheMode {
    /// The default CPU cache mode.
    MTLCPUCacheModeDefaultCache = 0,

    /// A write-combined CPU cache mode.
    MTLCPUCacheModeWriteCombined = 1
}

/// The storage mode used for the location and mapping of a resource.
///
/// # Discussion
///
/// Changes due to CPU stores outside of the Metal API must be indicated to the
/// application via the `didModifyRange:` method of `MTLBuffer`. In order for
/// the CPU to access up-to-date GPU results, a blit synchronization must be
/// completed via the `synchronizeResource:` or `synchronizeTexture:slice:level:`
/// method of `MTLBlitCommandEncoder`. Blit overhead is incurred only if the GPU
/// has modified the resource.
#[repr(usize)]
pub enum MTLStorageMode {
    /// The default storage mode for buffers.
    MTLStorageModeShared  = 0,

    /// The storage mode for a resource kept entirely in GPU memory.
    MTLStorageModePrivate = 2,
}

// TODO(burtonageo): Placeholder
pub const MTLResourceStorageModeShift: usize = 16;

// TODO(burtonageo): Placeholder
pub const MTLResourceCPUCacheModeShift: usize = 12;

/// Optional arguments used to create and influence behavior of buffer and
/// texture objects.
bitflags! {
    pub flags MTLResourceOptions: usize {
        /// The default CPU cache mode for the resource. Guarantees that read and
        /// write operations are executed in the expected order.
        const MTLResourceCPUCacheModeDefaultCache =
            (::MTLCPUCacheMode::MTLCPUCacheModeDefaultCache as usize) << ::MTLResourceCPUCacheModeShift,
        
        /// A write-combined CPU cache mode for the resource. Optimized for resources
        /// that the CPU will write into, but never read.
        const MTLResourceCPUCacheModeWriteCombined =
            (::MTLCPUCacheMode::MTLCPUCacheModeWriteCombined as usize) << ::MTLResourceCPUCacheModeShift,
        
        /// The CPU and GPU both use the same underlying memory when accessing the
        /// contents of the resource. Coherency is guaranteed only at command buffer
        /// boundaries to minimize the required flushing of CPU or GPU caches.
        const MTLResourceStorageModeShared = (::MTLStorageMode::MTLStorageModeShared as usize) <<
                                             ::MTLResourceStorageModeShift,

        /// The storage mode for a resource kept entirely in GPU memory. No coherency
        /// of any kind must be maintained in this mode because the resource will
        /// never be directly accessed by the CPU.
        const MTLResourceStorageModePrivate = (::MTLStorageMode::MTLStorageModePrivate as usize) <<
                                              ::MTLResourceStorageModeShift,
        
        /// The default CPU cache mode for general-purpose access to the storage
        /// allocation of the resource, which guarantees that read and write operations
        /// are executed in the expected order.
        ///
        /// This constant was deprecated in iOS 9.0 and OS X 10.11. Use
        /// `MTLResourceCPUCacheModeDefaultCache` instead.
        const MTLResourceOptionCPUCacheModeDefault = MTLResourceCPUCacheModeDefaultCache.bits,
        
        /// A CPU cache mode set to use write-combined memory for the storage allocation
        /// of the resource.
        ///
        /// This constant was deprecated in iOS 9.0 and OS X 10.11. Use
        /// `MTLResourceCPUCacheModeWriteCombined` instead.
        const MTLResourceOptionCPUCacheModeWriteCombined = MTLResourceCPUCacheModeWriteCombined.bits,
    }
}

/// The purgeable state of the resource.
#[repr(usize)]
pub enum MTLPurgeableState {
   MTLPurgeableStateKeepCurrent = 1,
   MTLPurgeableStateNonVolatile = 2,
   MTLPurgeableStateVolatile = 3,
   MTLPurgeableStateEmpty = 4
}
