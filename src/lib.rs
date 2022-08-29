#![deny(clippy::all)]

use napi_derive::napi;

#[napi(object)]
pub struct Plugin {
  pub name: String,
}

struct TransformResult {
  code: String
}


impl Plugin {
  fn transform(code: String) -> TransformResult {
      println!("transforming {code}");

      TransformResult { code }
    }
}

#[napi]
pub fn rollup_plugin_local_import() -> Plugin {
  Plugin { name: "local-import".to_string() }
}
