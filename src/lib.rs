use napi_derive::napi;

mod parser;

#[napi(object)]
pub struct TransformResult {
    pub code: String,
}

#[napi]
pub struct Plugin {
    pub name: String,
    pub extension: String,
}

#[napi]
impl Plugin {
    #[napi]
    pub fn transform(&self, source_code: String) -> TransformResult {
        let transformed = parser::parse(&source_code, &self.extension);

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
