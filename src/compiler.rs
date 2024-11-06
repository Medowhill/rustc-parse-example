use std::path::Path;

use rustc_ast::{
    visit::{FnKind, Visitor},
    NodeId,
};
use rustc_errors::registry::Registry;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_interface::Config;
use rustc_session::config::{CheckCfg, Input, Options};
use rustc_span::Span;

pub fn run(path: &Path) {
    let config = Config {
        opts: Options::default(),
        crate_cfg: FxHashSet::default(),
        crate_check_cfg: CheckCfg::default(),
        input: Input::File(path.to_owned()),
        output_dir: None,
        output_file: None,
        ice_file: rustc_driver::ice_path().clone(),
        file_loader: None,
        locale_resources: rustc_driver_impl::DEFAULT_LOCALE_RESOURCES,
        lint_caps: FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: Registry::new(rustc_error_codes::DIAGNOSTICS),
    };
    rustc_driver::catch_fatal_errors(|| {
        rustc_interface::run_compiler(config, |compiler| {
            compiler.enter(|queries| {
                queries.global_ctxt().unwrap().enter(|tcx| {
                    let sess = &tcx.sess.parse_sess;
                    let krate = rustc_parse::parse_crate_from_file(path, sess).unwrap();
                    rustc_ast::visit::walk_crate(&mut MyVisitor, &krate);
                })
            })
        })
    })
    .unwrap();
}

struct MyVisitor;

impl<'ast> Visitor<'ast> for MyVisitor {
    fn visit_fn(&mut self, fk: FnKind<'ast>, _: Span, _: NodeId) {
        if let FnKind::Fn(_, f, _, _, _, _) = fk {
            println!("{}", f.name.to_ident_string());
        }
        rustc_ast::visit::walk_fn(self, fk)
    }
}
