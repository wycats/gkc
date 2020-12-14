use regex::Regex;
use string_cache::Atom;
use swc_ecma_ast::{ImportDecl, NamedExport, Str};
use swc_ecma_visit::Fold;

pub struct RewriteExt;

impl Fold for RewriteExt {
    fn fold_named_export(&mut self, mut n: NamedExport) -> NamedExport {
        n.src = n.src.map(rewrite_filename);

        n
    }

    fn fold_import_decl(&mut self, mut n: ImportDecl) -> ImportDecl {
        n.src = rewrite_filename(n.src);
        n
    }
}

fn rewrite_filename(
    Str {
        span,
        value,
        has_escape,
    }: Str,
) -> Str {
    let src = value.as_ref();
    let ext = Regex::new(r"\.(ts|js)$").unwrap();

    let is_relative = src.starts_with('.');
    let has_ext = ext.is_match(src);

    if is_relative && !has_ext {
        Str {
            span,
            has_escape,
            value: Atom::from(format!("{}.js", src)),
        }
    } else {
        Str {
            span,
            has_escape,
            value,
        }
    }
}
