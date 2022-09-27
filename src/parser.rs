extern crate swc_common;

use swc::{config::Options, Compiler};
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    FileName, SourceMap,
};

use swc_ecma_ast::{ExportAll, ImportDecl, NamedExport};
use swc_ecma_visit::{as_folder, Fold};
use swc_ecmascript::{transforms::pass::noop, visit::VisitMut};

type Callback = Box<dyn Fn(String) -> String>;

struct Visitor {
    callback: Callback,
}

impl VisitMut for Visitor {
    fn visit_mut_export_all(&mut self, node: &mut ExportAll) {
        let path = node.src.value.to_string();

        if is_local_import(&path) {
            let new_path = (&self.callback)(path);
            node.src.value = new_path.into()
        }
    }

    fn visit_mut_named_export(&mut self, node: &mut NamedExport) {
        let path = node.src.as_ref().unwrap().value.to_string();

        if is_local_import(&path) {
            let new_path = (&self.callback)(path);
            node.src.as_mut().unwrap().value = new_path.into()
        }
    }

    fn visit_mut_import_decl(&mut self, node: &mut ImportDecl) {
        let path = node.src.value.to_string();

        if is_local_import(&path) {
            let new_path = (&self.callback)(path);
            node.src.value = new_path.into()
        }
    }
}

fn is_local_import(path: &str) -> bool {
    return path.starts_with("./") || path.starts_with("../");
}

fn visitor(callback: Callback) -> impl Fold {
    as_folder(Visitor { callback })
}

pub fn parse(code: &str, callback: Callback) -> String {
    let source_map: Lrc<SourceMap> = Default::default();
    let source_file =
        source_map.new_source_file(FileName::Custom("source.js".into()), code.to_string());
    let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));

    let compiler = Compiler::new(source_map);

    let transformed = compiler.process_js_with_custom_pass(
        source_file,
        None,
        &handler,
        &Options::default(),
        |_, _| visitor(callback),
        |_, _| noop(),
    );

    handler.abort_if_errors();

    // TODO: Include source map
    transformed.unwrap().code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_js_extension_callback() -> Callback {
        Box::new(|path: String| {
            let mut new_path: String = path.to_string();
            new_path.push_str(".js");

            new_path
        })
    }

    #[test]
    fn calls_callback_with_path() {
        parse(
            "export * from \"./local-file\";",
            Box::new(|path: String| {
                assert_eq!(path, "./local-file");
                path
            }),
        );
    }

    #[test]
    fn export_all_local_file_in_same_directory() {
        let source_code = "export * from \"./local-file\";";

        let transformed = parse(source_code, add_js_extension_callback());

        assert_eq!(transformed.trim(), "export * from \"./local-file.js\";");
    }

    #[test]
    fn export_all_local_file_in_parent_directory() {
        let source_code = "export * from \"../local-file\";";

        let transformed = parse(source_code, add_js_extension_callback());

        assert_eq!(transformed.trim(), "export * from \"../local-file.js\";");
    }

    #[test]
    fn export_all_dependency() {
        let source_code = "export * from \"some-dependency\";";

        let transformed = parse(source_code, add_js_extension_callback());

        assert_eq!(transformed.trim(), source_code);
    }

    #[test]
    fn export_named_local_file_in_same_directory() {
        let source_code = "export { method } from \"./local-file\";";

        let transformed = parse(source_code, add_js_extension_callback());

        assert_eq!(
            transformed.trim(),
            "export { method } from \"./local-file.js\";"
        );
    }

    #[test]
    fn export_named_local_file_in_parent_directory() {
        let source_code = "export { method } from \"../local-file\";";

        let transformed = parse(source_code, add_js_extension_callback());

        assert_eq!(
            transformed.trim(),
            "export { method } from \"../local-file.js\";"
        );
    }

    #[test]
    fn export_named_dependency() {
        let source_code = "export { method } from \"some-dependency\";";

        let transformed = parse(source_code, add_js_extension_callback());

        assert_eq!(transformed.trim(), source_code);
    }
}
