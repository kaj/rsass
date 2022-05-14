//! Tests auto-converted from "sass-spec/spec/css/plain/import"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("import")
}

mod conditions;

mod css_before_index;

mod in_css;

mod partial_conflict;

mod sass_takes_precedence;

mod scss_takes_precedence;
