//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("complex")
}

mod combinators;

mod distinct;

mod identical;

mod lcs;

mod overlap;

mod rootish;

mod superselector;
