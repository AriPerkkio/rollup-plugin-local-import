#![deny(clippy::all)]

use napi_derive::napi;

mod parser;

#[napi(constructor)]
pub struct Plugin {
    pub name: String,
}

#[napi(object)]
pub struct TransformResult {
    pub code: String,
}

#[napi]
impl Plugin {
    #[napi]
    pub fn transform(&self, source_code: String) -> TransformResult {
        let transformed = parser::parse(&source_code);

        // TODO: Include source map
        TransformResult { code: transformed }
    }
}

#[napi]
pub fn local_import() -> Plugin {
    Plugin {
        name: "local-import".to_string(),
    }
}
