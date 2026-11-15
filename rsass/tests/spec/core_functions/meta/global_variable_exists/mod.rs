//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("global_variable_exists")
}

mod dash_insensitive;

mod different_module;

mod error;

mod escape;

mod named;

mod same_module;
