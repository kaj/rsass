//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("get_function")
}

mod different_module;

mod equality;

mod error;

mod meta;

mod same_module;

mod scope;
