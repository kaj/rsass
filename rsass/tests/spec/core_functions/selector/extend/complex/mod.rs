//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/complex"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("complex")
}

mod combinator_only;

mod with_unification;

mod without_unification;
