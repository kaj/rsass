//! Tests auto-converted from "sass-spec/spec/directives/forward/with"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with")
}

mod core_module;

mod dash_insensitive;

mod default;

mod doesnt_run_default;

mod facade_contains_multiple_configured_forwards;

mod from_variable;

mod multi_load;

mod multiple;

mod null;

mod single;

mod some_unconfigured;

mod through_forward;

mod through_import;

mod trailing_comma;

mod used_in_input;

mod variable_exists;
