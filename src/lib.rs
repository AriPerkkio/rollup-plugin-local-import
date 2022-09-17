use neon::prelude::*;

mod parser;

struct TransformResult {
    pub code: String,
}

impl TransformResult {
    // https://neon-bindings.com/docs/objects#converting-rust-data-to-javascript
    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let code = cx.string(&self.code);
        obj.set(cx, "code", code)?;

        Ok(obj)
    }
}

fn transform(mut cx: FunctionContext) -> JsResult<JsObject> {
    let extension = cx.argument::<JsString>(0)?;
    let source_code = cx.argument::<JsString>(1)?;

    let transformed = parser::parse(&source_code.value(&mut cx), &extension.value(&mut cx));

    // TODO: Include source map
    let result = TransformResult { code: transformed };

    let output = result.to_object(&mut cx)?;

    Ok(output)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let name: Handle<JsString> = cx.string("local-import");
    cx.export_value("name", name)?;

    cx.export_function("transform", transform)?;

    Ok(())
}
