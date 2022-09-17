mod parser;

pub fn transform(source_code: String) -> TransformResult {
    let transformed = parser::parse(&source_code);

    // TODO: Include source map
    TransformResult { code: transformed }
}

pub struct TransformResult {
    pub code: String,
}
