//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/media/nesting"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nesting")
}

mod merged;

mod merged_and_retained;

mod removed;

mod retained;
