//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("load")
}

mod error;

mod live;

mod no_error;

mod shares_state;

mod with;
