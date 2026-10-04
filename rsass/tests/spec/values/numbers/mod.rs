//! Tests auto-converted from "sass-spec/spec/values/numbers"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("numbers")
}

mod bounds;

mod degenerate;

mod divide;

mod error;

mod modulo;

mod negative_zero;

mod precision;

mod units;

mod very_large;
