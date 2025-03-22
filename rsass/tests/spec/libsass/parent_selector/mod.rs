//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parent-selector")
}

mod basic;

mod inner_combinator;

mod inner_pseudo;

mod missing;

mod outer_combinator;

mod outer_pseudo;
