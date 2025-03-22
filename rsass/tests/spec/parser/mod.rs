//! Tests auto-converted from "sass-spec/spec/parser"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parser")
}

mod interpolation;

mod operator_precedence;

mod selector;
