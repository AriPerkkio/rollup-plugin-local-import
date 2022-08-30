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
    pub fn transform(&self, code: String) -> TransformResult {
        parser::parse(&code);

        TransformResult { code }
    }
}

#[napi]
pub fn local_import() -> Plugin {
    Plugin {
        name: "local-import".to_string(),
    }
}
