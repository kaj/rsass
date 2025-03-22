//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("pow")
}

mod arguments;

mod base_greater_than_zero;

mod base_less_than_zero;

mod base_negative_infinity;

mod base_negative_zero;

mod base_positive_infinity;

mod base_positive_zero;
