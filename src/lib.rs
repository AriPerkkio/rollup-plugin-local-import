mod parser;

pub struct TransformResult {
    pub code: String,
}

pub const NAME: &str = "local-import";

pub fn transform(extension: &str, source_code: &str) -> TransformResult {
    let transformed = parser::parse(source_code, extension);

    // TODO: Include source map
    TransformResult { code: transformed }
}
