extern crate swc_common;

use swc::{config::Options, Compiler};
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    FileName, SourceMap
};

use swc_ecma_ast::{ExportAll};
use swc_ecma_visit::{as_folder, Fold};
use swc_ecmascript::{transforms::pass::noop, visit::VisitMut};

struct Visitor;
impl VisitMut for Visitor {
    fn visit_mut_export_all(&mut self, node: &mut ExportAll) {
        println!("Visiting ExportAll - {:?}", node);

        let mut path = node.src.value.to_string();
        println!("Original path is {:?}", path);

        path.push_str(".js");
        println!("New path is {:?}", path);

        node.src.value = path.into()
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
