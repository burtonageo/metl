use cocoa::base::id;
use MTLLanguageVersion;

pub trait MTLCompileOptions {
    /// A Boolean value that indicates whether the compiler can perform optimizations for
    /// floating-point arithmetic that may violate the IEEE 754 standard. A YES value also
    /// enables the high-precision variant of math functions for single-precision floating-point
    /// scalar and vector types.
    ///
    /// # Discussion
    ///
    /// The default value is YES.
    unsafe fn fastMathEnabled(self) -> bool;
    unsafe fn setFastMathEnabled(self, bool);

    /// The language version used to interpret the library source code.
    ///
    /// # Discussion
    ///
    /// By default, the most recent language version available is used.
    unsafe fn languageVersion(self) -> MTLLanguageVersion;
    unsafe fn setLanguageVersion(self, MTLLanguageVersion);

    /// A list of preprocessor macros to consider when compiling this code.
    ///
    /// # Discussion
    ///
    /// The macros are specified as key/value pairs, using an NSDictionary. The keys must be
    /// NSString objects, and the values can be either NSString or NSNumber objects.
    ///
    /// The default value is nil.
    unsafe fn preprocessorMacros(self) -> id;
    unsafe fn setPreprocessorMacros(self, id);
}

