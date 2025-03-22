//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("variable-scoping")
}

mod blead_global;

mod defaults_global_null;

mod defaults_global;

mod defaults_null;

mod defaults;

mod feature_test;

mod lexical_scope;

mod root_scope;
