//! Tests auto-converted from "sass-spec/spec/css/selector"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector")
}

mod attribute;

mod combinator;

mod escaping;

mod placeholder;

mod pseudoselector;

mod reference_combinator;

mod slotted;
