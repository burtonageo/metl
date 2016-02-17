use cocoa::base::{id, nil};
use cocoa::foundation::{NSString, NSUInteger};
use core_foundation::base::TCFType;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use objc::runtime::BOOL;
use objc_bringup::NSDictionary;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::default::Default;
use sys::{MTLCompileOptions, MTLLanguageVersion};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompileOptions {
    /// Is fast math enabled. Set this option to `None` to use the system default setting.
    pub fast_math_enabled: Option<bool>,

    /// The language version used. Either use the latest by default, or set this to a specific
    /// version.
    pub language_version: LanguageVersion,

    /// A map of preprocessor macro names to values. Each entry of (`MacroName`, `MacroValue`)
    /// corresponds to `#define MacroName MacroValue`
    pub preprocessor_macros: HashMap<String, PreprocessorMacroValue>
}

impl CompileOptions {
    pub fn fast_math_enabled(self, fast_math: bool) -> Self {
        CompileOptions { fast_math_enabled: Some(fast_math), ..self }
    }

    pub fn language_version(self, language_version: LanguageVersion) -> Self {
        CompileOptions { language_version: language_version, ..self }
    }

    pub fn with_macro<S: Into<String>, M: Into<PreprocessorMacroValue>>(mut self, mac_name: S,
                                                                        mac_val: M)
                                                                        -> Self {
        self.preprocessor_macros.insert(mac_name.into(), mac_val.into());
        self
    }

    pub fn mtl_compile_options(&self) -> id {
        unsafe {
            let ll_compile_opts = MTLCompileOptions::new(nil);

            if let Some(fast_math_enabled) = self.fast_math_enabled {
                ll_compile_opts.setFastMathEnabled(fast_math_enabled as BOOL);
            }

            if let LanguageVersion::Specific(version) = self.language_version {
                ll_compile_opts.setLanguageVersion(version.into());
            }

            if !self.preprocessor_macros.is_empty() {
                let mut keys = self.preprocessor_macros
                                   .keys()
                                   .map(|k| NSString::alloc(nil).init_str(k.as_ref()))
                                   .collect::<Vec<_>>();

                let mut objs = self.preprocessor_macros
                                   .values()
                                   .map(|v| {
                                       match v {
                                           &PreprocessorMacroValue::Floating(f) => {
                                               CFNumber::from_f64(f).as_CFTypeRef() as id
                                           }
                                           &PreprocessorMacroValue::Integer(i) => {
                                               CFNumber::from_i64(i).as_CFTypeRef() as id
                                           }
                                           &PreprocessorMacroValue::String(ref s) => {
                                               CFString::new(s.as_ref()).as_CFTypeRef() as id
                                           }
                                       }
                                   })
                                   .collect::<Vec<_>>();

                let num_macros = self.preprocessor_macros.len() as NSUInteger;

                let macros = NSDictionary::dictionaryWithObjects_forKeys_count(objs.as_mut_ptr(),
                                                                               keys.as_mut_ptr(),
                                                                               num_macros);
                ll_compile_opts.setPreprocessorMacros(macros);
            }

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
    Floating(f64),
    Integer(i64),
    String(String)
}

impl From<i64> for PreprocessorMacroValue {
    fn from(val: i64) -> Self {
        PreprocessorMacroValue::Integer(val)
    }
}

impl From<i32> for PreprocessorMacroValue {
    fn from(val: i32) -> Self {
        PreprocessorMacroValue::Integer(val as i64)
    }
}

impl From<isize> for PreprocessorMacroValue {
    fn from(val: isize) -> Self {
        PreprocessorMacroValue::Integer(val as i64)
    }
}

impl From<f64> for PreprocessorMacroValue {
    fn from(val: f64) -> Self {
        PreprocessorMacroValue::Floating(val)
    }
}

impl From<f32> for PreprocessorMacroValue {
    fn from(val: f32) -> Self {
        PreprocessorMacroValue::Floating(val as f64)
    }
}

impl From<String> for PreprocessorMacroValue {
    fn from(val: String) -> Self {
        PreprocessorMacroValue::String(val)
    }
}

impl<'a> From<&'a str> for PreprocessorMacroValue {
    fn from(val: &'a str) -> Self {
        PreprocessorMacroValue::String(val.into())
    }
}
