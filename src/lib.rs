#![deny(clippy::all)]

use napi_derive::napi;

mod parser;

// This constant is intentionally lowercase as it represents rollup plugin's "name" property
#[allow(non_upper_case_globals)]
#[napi]
pub const name: &str = "local-import";

#[napi]
pub fn transform(source_code: String) -> TransformResult {
    let transformed = parser::parse(&source_code);

    // TODO: Include source map
    TransformResult { code: transformed }
}

#[napi(object)]
pub struct TransformResult {
    pub code: String,
}
