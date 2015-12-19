use cocoa::base::{id, nil};
use objc::runtime::BOOL;
use std::collections::HashMap;
use sys::MTLCompileOptions;

#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub fast_math_enabled: bool,
    pub language_version: (), // Todo(George): Make this field the right type
    pub preprocessor_macros: HashMap<String, PreprocessorMacroValue>
}

impl CompileOptions {
    #[allow(unreachable_code)]
    pub fn into_mtl_compile_options(self) -> id {
        unsafe {
            let ll_compile_opts = MTLCompileOptions::new(nil);
            ll_compile_opts.setFastMathEnabled(self.fast_math_enabled as BOOL);
            // Todo(George): Set the rest of the compile options correctly

            unimplemented!();

            ll_compile_opts
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PreprocessorMacroValue {
    Numeric(f64),
    String(String)
}
