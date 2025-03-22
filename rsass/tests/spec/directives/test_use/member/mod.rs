//! Tests auto-converted from "sass-spec/spec/directives/use/member"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("member")
}

mod global;

mod namespaced;

mod nested_global_variable;

mod use_to_import;
