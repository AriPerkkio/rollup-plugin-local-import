extern crate swc_common;

use swc::{config::Options, Compiler};
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    FileName, SourceMap,
};

use swc_ecma_ast::{ExportAll, NamedExport};
use swc_ecma_visit::{as_folder, Fold};
use swc_ecmascript::{transforms::pass::noop, visit::VisitMut};

struct Visitor;
impl VisitMut for Visitor {
    fn visit_mut_export_all(&mut self, node: &mut ExportAll) {
        let mut path = node.src.value.to_string();

        if path.starts_with("./") || path.starts_with("../") {
            path.push_str(".js");
            node.src.value = path.into()
        }
    }

    fn visit_mut_named_export(&mut self, node: &mut NamedExport) {
        let mut path = node.src.as_ref().unwrap().value.to_string();

        if path.starts_with("./") || path.starts_with("../") {
            path.push_str(".js");
            node.src.as_mut().unwrap().value = path.into()
        }
    }
}

fn visitor() -> impl Fold {
    as_folder(Visitor)
}

pub fn parse(code: &str) -> String {
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
        |_, _| visitor(),
        |_, _| noop(),
    );

    handler.abort_if_errors();

    // TODO: Include source map
    transformed.unwrap().code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_all_local_file_in_same_directory() {
        let source_code = "export * from \"./local-file\";";

        let transformed = parse(source_code);

        assert_eq!(transformed.trim(), "export * from \"./local-file.js\";");
    }

    #[test]
    fn export_all_local_file_in_parent_directory() {
        let source_code = "export * from \"../local-file\";";

        let transformed = parse(source_code);

        assert_eq!(transformed.trim(), "export * from \"../local-file.js\";");
    }

    #[test]
    fn export_all_dependency() {
        let source_code = "export * from \"some-dependency\";";

        let transformed = parse(source_code);

        assert_eq!(transformed.trim(), source_code);
    }

    #[test]
    fn export_named_local_file_in_same_directory() {
        let source_code = "export { method } from \"./local-file\";";

        let transformed = parse(source_code);

        assert_eq!(
            transformed.trim(),
            "export { method } from \"./local-file.js\";"
        );
    }

    #[test]
    fn export_named_local_file_in_parent_directory() {
        let source_code = "export { method } from \"../local-file\";";

        let transformed = parse(source_code);

        assert_eq!(
            transformed.trim(),
            "export { method } from \"../local-file.js\";"
        );
    }

    #[test]
    fn export_named_dependency() {
        let source_code = "export { method } from \"some-dependency\";";

        let transformed = parse(source_code);

        assert_eq!(transformed.trim(), source_code);
    }
}
