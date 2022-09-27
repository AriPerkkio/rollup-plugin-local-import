use napi::{Env, JsFunction, JsString, Ref};
use napi_derive::napi;

mod parser;

#[napi(object)]
pub struct TransformResult {
    pub code: String,
}

#[napi]
pub struct Plugin {
    pub name: String,
    callback: Ref<()>,
}

#[napi]
impl Plugin {
    #[napi]
    pub fn transform(&self, env: Env, source_code: String) -> TransformResult {
        let callback: JsFunction = env.get_reference_value(&self.callback).unwrap();

        let transform_paths = Box::new(move |path: String| {
            let args: [JsString; 1] = [env.create_string(&path).unwrap()];

            let modified = callback
                .call(None, &args)
                .unwrap()
                .coerce_to_string()
                .unwrap()
                .into_utf8()
                .unwrap();

            return modified.as_str().unwrap().to_string();
        });

        let transformed = parser::parse(&source_code, transform_paths);

        // TODO: Include source map
        TransformResult { code: transformed }
    }

    #[napi]
    pub fn build_end(&mut self, env: Env) {
        self.callback.unref(env).unwrap();
    }
}

#[napi(ts_args_type = "callback: (path: string) => string")]
pub fn local_import(env: Env, callback: JsFunction) -> Plugin {
    Plugin {
        name: String::from("local-import"),
        callback: env.create_reference(callback).unwrap(),
    }
}
