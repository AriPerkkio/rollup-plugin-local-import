use std::mem;

use napi_derive::napi;

mod parser;

#[napi(object)]
pub struct TransformResult {
    pub code: String,
}

#[napi]
pub struct Plugin {
    pub name: String,
    extension: String,
}

#[napi]
impl Plugin {
    #[napi]
    pub fn transform(&self, source_code: String) -> TransformResult {
        // Oh no, don't look here
        let extension: &'static str = Box::leak(self.extension.to_string().into_boxed_str());

        let callback = Box::new(|path: String| {
            let mut new_path = path;
            new_path.push_str(extension);

            new_path
        });

        let transformed = parser::parse(&source_code, callback);
        mem::forget(extension);

        // TODO: Include source map
        TransformResult { code: transformed }
    }
}

#[napi]
pub fn local_import(extension: String) -> Plugin {
    Plugin {
        name: String::from("local-import"),
        extension,
    }
}
