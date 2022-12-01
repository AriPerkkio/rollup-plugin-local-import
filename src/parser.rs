use swc_core::{
    base::{
        config::{Options, SourceMapsConfig},
        Compiler, TransformOutput,
    },
    common::{
        errors::{ColorConfig, Handler},
        sync::Lrc,
        FileName, SourceMap,
    },
    ecma::{
        transforms::base::pass::noop,
        visit::{
            as_folder,
            swc_ecma_ast::{ExportAll, ImportDecl, NamedExport},
            Fold, VisitMut,
        },
    },
};

use crate::TransformResult;

type Callback = Box<dyn Fn(String) -> Result<String, String>>;

struct Visitor {
    callback: Callback,
    errors: Vec<String>,
}

impl VisitMut for Visitor {
    fn visit_mut_export_all(&mut self, node: &mut ExportAll) {
        let path = node.src.value.to_string();

        if is_local_import(&path) {
            match (&self.callback)(path) {
                Ok(new_path) => node.src.value = new_path.into(),
                Err(error) => self.errors.push(error),
            }
        }
    }

    fn visit_mut_named_export(&mut self, node: &mut NamedExport) {
        let path = match node.src.as_ref() {
            // E.g. "export { method } from './local-file';", src is "./local-file"
            Some(src) => src.value.to_string(),

            // E.g. "export { SomeVariableInScope };", src is None
            None => return,
        };

        if is_local_import(&path) {
            match (&self.callback)(path) {
                Ok(new_path) => node.src.as_mut().unwrap().value = new_path.into(),
                Err(error) => self.errors.push(error),
            };
        }
    }

    fn visit_mut_import_decl(&mut self, node: &mut ImportDecl) {
        let path = node.src.value.to_string();

        if is_local_import(&path) {
            match (&self.callback)(path) {
                Ok(new_path) => node.src.value = new_path.into(),
                Err(error) => self.errors.push(error),
            };
        }
    }
}

fn is_local_import(path: &str) -> bool {
    return path.starts_with("./") || path.starts_with("../");
}

fn as_visitor(visitor: &mut Visitor) -> impl Fold + '_ {
    as_folder(visitor)
}

pub fn parse(code: &str, filename: &str, callback: Callback) -> Result<TransformResult, String> {
    let source_map: Lrc<SourceMap> = Default::default();
    let source_file =
        source_map.new_source_file(FileName::Custom(filename.to_string()), code.to_string());
    let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));

    let compiler = Compiler::new(source_map);

    let mut visitor = Visitor {
        callback,
        errors: vec![],
    };

    let transformed = compiler.process_js_with_custom_pass(
        source_file,
        None,
        &handler,
        &Options {
            source_file_name: Some(filename.to_string()),
            source_maps: Some(SourceMapsConfig::Bool(true)),
            ..Default::default()
        },
        |_, _| as_visitor(&mut visitor),
        |_, _| noop(),
    );

    if visitor.errors.len() > 0 {
        return Err(format!(
            "Run into {:?} error(s): [{:?}].",
            visitor.errors.len(),
            visitor.errors.join(",")
        ));
    }

    handler.abort_if_errors();

    let TransformOutput {
        code,
        map: option_map,
    } = match transformed {
        Ok(result) => result,
        Err(error) => {
            return Err(error.to_string());
        }
    };

    match option_map {
        Some(map) => Ok(TransformResult { code, map }),
        None => Err("Failed to generate sourcemaps.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_js_extension_callback() -> Callback {
        Box::new(|path: String| {
            let mut new_path: String = path.to_string();
            new_path.push_str(".js");

            Ok(new_path)
        })
    }

    #[test]
    fn calls_callback_with_path() {
        parse(
            "export * from \"./local-file\";",
            "some-file.js",
            Box::new(|path: String| {
                assert_eq!(path, "./local-file");
                Ok(path)
            }),
        )
        .unwrap();
    }

    #[test]
    fn export_all_local_file_in_same_directory() {
        let source_code = "export * from \"./local-file\";";

        let transformed = parse(source_code, "some-file.js", add_js_extension_callback())
            .unwrap()
            .code;

        assert_eq!(transformed.trim(), "export * from \"./local-file.js\";");
    }

    #[test]
    fn export_all_local_file_in_parent_directory() {
        let source_code = "export * from \"../local-file\";";

        let transformed = parse(source_code, "some-file.js", add_js_extension_callback())
            .unwrap()
            .code;

        assert_eq!(transformed.trim(), "export * from \"../local-file.js\";");
    }

    #[test]
    fn export_all_dependency() {
        let source_code = "export * from \"some-dependency\";";

        let transformed = parse(source_code, "some-file.js", add_js_extension_callback())
            .unwrap()
            .code;

        assert_eq!(transformed.trim(), source_code);
    }

    #[test]
    fn export_named_local_file_in_same_directory() {
        let source_code = "export { method } from \"./local-file\";";

        let transformed = parse(source_code, "some-file.js", add_js_extension_callback())
            .unwrap()
            .code;

        assert_eq!(
            transformed.trim(),
            "export { method } from \"./local-file.js\";"
        );
    }

    #[test]
    fn export_named_local_file_in_parent_directory() {
        let source_code = "export { method } from \"../local-file\";";

        let transformed = parse(source_code, "some-file.js", add_js_extension_callback())
            .unwrap()
            .code;

        assert_eq!(
            transformed.trim(),
            "export { method } from \"../local-file.js\";"
        );
    }

    #[test]
    fn export_named_dependency() {
        let source_code = "export { method } from \"some-dependency\";";

        let transformed = parse(source_code, "some-file.js", add_js_extension_callback())
            .unwrap()
            .code;

        assert_eq!(transformed.trim(), source_code);
    }

    #[test]
    fn re_export_named_import() {
        let source_code = "
        import { sideEffects } from \"./some-file\";
        sideEffects();
        export { sideEffects };
        ";

        let transformed = parse(source_code, "some-file.js", add_js_extension_callback())
            .unwrap()
            .code;

        assert_eq!(
            transformed.trim(),
            "import { sideEffects } from \"./some-file.js\";
sideEffects();
export { sideEffects };"
        );
    }
}
