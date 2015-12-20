use cocoa::base::{id, nil};
use objc::runtime::BOOL;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::default::Default;
use sys::{MTLCompileOptions, MTLLanguageVersion};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompileOptions {
    pub fast_math_enabled: bool,
    pub language_version: LanguageVersion,
    pub preprocessor_macros: HashMap<String, PreprocessorMacroValue>
}

impl CompileOptions {
    #[allow(unreachable_code)]
    pub fn mtl_compile_options(&self) -> id {
        unsafe {
            let ll_compile_opts = MTLCompileOptions::new(nil);

            ll_compile_opts.setFastMathEnabled(self.fast_math_enabled as BOOL);
            if let LanguageVersion::Specific(version) = self.language_version {
                ll_compile_opts.setLanguageVersion(version.into());
            }

            // TODO(George): Set the rest of the compile options correctly

            unimplemented!();

            ll_compile_opts
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LanguageVersion {
    Latest,
    Specific(SpecificLanguageVersion)
}

impl Default for LanguageVersion {
    fn default() -> Self {
        LanguageVersion::Latest
    }
}

/// Used to choose a specific version of the Metal shader language to use
// An underscore is a good separator for numbers
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpecificLanguageVersion {
    /// Version 1.0
    Version_1_0,

    /// Version 1.1
    Version_1_1
}

impl From<MTLLanguageVersion> for SpecificLanguageVersion {
    fn from(language_version: MTLLanguageVersion) -> Self {
        match language_version {
            MTLLanguageVersion::MTLLanguageVersion1_0 => SpecificLanguageVersion::Version_1_0,
            MTLLanguageVersion::MTLLanguageVersion1_1 => SpecificLanguageVersion::Version_1_1,
        }
    }
}

impl Into<MTLLanguageVersion> for SpecificLanguageVersion {
    fn into(self) -> MTLLanguageVersion {
        match self {
            SpecificLanguageVersion::Version_1_0 => MTLLanguageVersion::MTLLanguageVersion1_0,
            SpecificLanguageVersion::Version_1_1 => MTLLanguageVersion::MTLLanguageVersion1_1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PreprocessorMacroValue {
    Numeric(f64),
    String(String)
}
