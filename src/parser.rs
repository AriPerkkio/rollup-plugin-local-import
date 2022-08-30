extern crate swc_common;
extern crate swc_ecma_parser;

use swc::Compiler;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};
use swc_ecma_ast::{EsVersion, ExportAll, ExportDecl, ExportNamedSpecifier};
use swc_ecma_parser::Syntax;
use swc_ecma_visit::Fold;

struct Visitor {}

impl Fold for Visitor {
    fn fold_export_all(&mut self, n: ExportAll) -> ExportAll {
        println!("Visiting ExportAll - {:?}", n);
        return n;
    }
    fn fold_export_decl(&mut self, n: ExportDecl) -> ExportDecl {
        println!("Visiting ExportDecl - {:?}", n);
        return n;
    }
    fn fold_export_named_specifier(&mut self, n: ExportNamedSpecifier) -> ExportNamedSpecifier {
        println!("Visiting ExportNamedSpecifier - {:?}", n);
        return n;
    }
}

pub fn parse(code: &str) {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let fm = cm.new_source_file(FileName::Custom("test.js".into()), code.to_string());

    let comp = Compiler::new(cm);
    let _js = comp
        .parse_js(
            fm,
            &handler,
            EsVersion::Es2022,
            Syntax::Es(Default::default()),
            swc::config::IsModule::Bool(true),
            None,
        )
        .unwrap();

    let _trns = comp.transform(&handler, _js, false, Visitor {});
}
