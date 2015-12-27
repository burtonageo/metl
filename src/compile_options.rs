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

#[derive(Clone, Debug, PartialEq)]
pub struct CompileOptions {
    pub fast_math_enabled: bool,
    pub language_version: LanguageVersion,
    pub preprocessor_macros: HashMap<String, PreprocessorMacroValue>
}

impl CompileOptions {
    pub fn mtl_compile_options(&self) -> id {
        unsafe {
            let ll_compile_opts = MTLCompileOptions::new(nil);

            ll_compile_opts.setFastMathEnabled(self.fast_math_enabled as BOOL);

            if let LanguageVersion::Specific(version) = self.language_version {
                ll_compile_opts.setLanguageVersion(version.into());
            }

            if !self.preprocessor_macros.is_empty() {
                let mut keys = self.preprocessor_macros
                                   .keys()
                                   .map(|k| NSString::alloc(nil).init_str(k.as_ref()))
                                   .collect::<Vec<_>>();

                let mut objs =
                    self.preprocessor_macros
                        .values()
                        .map(|v| {
                            match v {
                                &PreprocessorMacroValue::Floating(f) => {
                                    CFNumber::from_f64(f).as_CFTypeRef() as id
                                }
                                &PreprocessorMacroValue::Integral(i) => {
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

impl Default for CompileOptions {
    fn default() -> Self {
        CompileOptions {
            fast_math_enabled: true,
            language_version: Default::default(),
            preprocessor_macros: Default::default()
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
    Integral(i64),
    String(String)
}
