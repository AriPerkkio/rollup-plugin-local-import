use napi::{Env, Error, JsFunction, JsString, Ref};
use napi_derive::napi;

mod parser;

#[napi(object)]
pub struct TransformResult {
    pub code: String,
    pub map: String,
}

#[napi]
pub struct Plugin {
    /// Name of the Rollup plugin https://rollupjs.org/guide/en/#name
    pub name: String,

    callback_reference: Ref<()>,
}

#[napi]
impl Plugin {
    /// Build hook: https://rollupjs.org/guide/en/#transform
    #[napi]
    pub fn transform(
        &self,
        env: Env,
        source_code: String,
        filename: String,
    ) -> Result<TransformResult, Error> {
        let callback: JsFunction = env.get_reference_value(&self.callback_reference).unwrap();

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

        parser::parse(&source_code, &filename, transform_paths)
    }

    /// Build hook: https://rollupjs.org/guide/en/#buildend
    #[napi]
    pub fn build_end(&mut self, env: Env) -> Result<(), Error> {
        if let Err(_) = self.callback_reference.unref(env) {
            return Err(Error::new(
                napi::Status::GenericFailure,
                String::from("Failed to cleanup callback. Unexpected Rollup lifecycle order."),
            ));
        }

        Ok(())
    }
}

#[napi(ts_args_type = "callback: (path: string) => string")]
pub fn local_import(env: Env, callback: JsFunction) -> Result<Plugin, Error> {
    let callback_reference = match env.create_reference(callback) {
        Ok(reference) => reference,
        Err(_) => {
            return Err(Error::new(
                napi::Status::GenericFailure,
                String::from("Failed to reference callback. Did you pass function to `localImport(callback)`?"),
            ));
        }
    };

    Ok(Plugin {
        name: String::from("local-import"),
        callback_reference,
    })
}
