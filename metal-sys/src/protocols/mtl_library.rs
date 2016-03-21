use cocoa::base::id;

/// The `MTLLibrary` protocol defines the interface for an object that represents a library of Metal
/// shader functions. A `MTLLibrary` object can contain Metal shading language code that is compiled
/// during the app build process or at runtime from a text string containing Metal shading language
/// source code. Your app does not define classes that implement this protocol.
///
/// Use a `MTLDevice` method (not standard allocation and initialization techniques) to create a
/// `MTLLibrary` object. To create a `MTLLibrary` object from a Metal library binary, call one of
/// these MTLDevice methods:
///
/// * `newDefaultLibrary`
///
/// * `newLibraryWithFile:error:`
///
/// * `newLibraryWithData:error:`
///
/// To create a `MTLLibrary` object by compiling source code, call one of these `MTLDevice` methods:
///
/// * `newLibraryWithSource:options:error:`
///
/// * `newLibraryWithSource:options:completionHandler:`
///
/// The `newFunctionWithName:` method is used to fetch functions from the library, which makes that
/// code available as a shader for either a `MTLRenderPipelineState` object for a render command
/// encoder or for a `MTLComputePipelineState` for a compute command encoder.
pub trait MTLLibrary {
    /// Returns a function object that represents an entry point in the library.
    ///
    /// # Parameters
    ///
    /// * `functionName` - The name of an entry point.
    ///
    /// # Return Value
    ///
    /// A function object for the named entry point, or nil if the named function is not found in
    /// the library.
    unsafe fn newFunctionWithName(self, functionName: id) -> id;

    /// A list of all entry points in the library. (read-only)
    ///
    /// # Discussion
    ///
    /// An array of `NSString` objects. Each string is the name of an entry point.
    unsafe fn functionNames(self) -> id;

    /// The device from which this library was created. (read-only)
    ///
    /// # Discussion
    ///
    /// This library can only be used with this device.
    unsafe fn device(self) -> id;

    /// A string to help identify the library object.
    unsafe fn label(self) -> id;
    unsafe fn setLabel(self, id);
}

impl MTLLibrary for id {
    unsafe fn newFunctionWithName(self, functionName: id) -> id {
        msg_send![self, newFunctionWithName:functionName]
    }

    unsafe fn functionNames(self) -> id {
        msg_send![self, functionNames]
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
}

/// Error conditions that can result from the creation of a `MTLLibrary` or `MTLFunction` object.
#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLLibraryError {
    /// The action is unsupported. For example, the requested library file has improper formatting,
    /// or the requested library is not accessible.
    MTLLibraryErrorUnsupported = 1,

    /// The action causes an internal error.
    MTLLibraryErrorInternal = 2,

    /// Compilation fails
    MTLLibraryErrorCompileFailure = 3,

    /// Compilation succeeds without error, but there are compiler warnings.
    MTLLibraryErrorCompileWarning = 4
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    /// Constant to identify the `MTLLibrary` error domain.
    pub static MTLLibraryErrorDomain: id;
}

/// Error conditions that can result from the creation of a `MTLRenderPipelineState` object.
#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLRenderPipelineError {
    /// The action causes an internal error.
    MTLRenderPipelineErrorInternal = 1,

    /// The action is unsupported.
    MTLRenderPipelineErrorUnsupported = 2,

    /// The input values are invalid.
    MTLRenderPipelineErrorInvalidInput = 3
}

#[link(name = "Metal", kind = "framework")]
extern "C" {
    /// Constant to identify the `MTLRenderPipelineState` error domain.
    pub static MTLRenderPipelineErrorDomain: id;
}


#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MTLLanguageVersion {
    /// Version 1.0
    MTLLanguageVersion1_0 = (1 << 16),

    /// Version 1.1
    MTLLanguageVersion1_1 = (1 << 16) + 1
}
