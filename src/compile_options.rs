use cocoa::base::id;
use std::collections::HashMap;
#[allow(unused_imports)]
use sys::MTLCompileOptions;

pub struct CompileOptions {
    pub fast_math_enabled: bool,
    pub language_version: (), // Todo(George): Make this field the right type
    pub preprocessor_macros: HashMap<String, PreprocessorMacroValue>
}

impl CompileOptions {
    pub fn into_mtl_compile_options(self) -> id {
        unimplemented!();
    }
}

pub enum PreprocessorMacroValue {
    Numeric(f64),
    String(String)
}
