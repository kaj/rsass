//! Tests auto-converted from "sass-spec/spec/directives/use/extend"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("extend")
}

mod diamond;

mod extended;

mod midstream_extend_within_pseudoselector;

mod optional_and_mandatory;

mod scope;

mod upstream;
