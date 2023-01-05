//! Tests auto-converted from "sass-spec/spec/directives/use/with"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with")
}

mod core_module;

mod dash_insensitive;

mod distributed_vars;

mod doesnt_run_default;

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
