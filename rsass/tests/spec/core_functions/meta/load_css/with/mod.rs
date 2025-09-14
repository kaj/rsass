//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with")
}

mod core_module;

mod dash_insensitive;

mod doesnt_run_default;

mod empty;

mod multi_load;

mod multiple;

mod null;

mod single;

mod some_unconfigured;

mod through_forward;

mod through_import;

mod variable_exists;
