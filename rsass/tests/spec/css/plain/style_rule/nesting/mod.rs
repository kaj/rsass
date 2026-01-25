//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nesting")
}

mod combinator;

mod media;

mod multiple_complex;

mod one_level;

mod parent;

mod supports;

mod through_import;

mod through_load_css;

mod two_levels;

mod unknown;

mod with_declaration;
