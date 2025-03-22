//! Tests auto-converted from "sass-spec/spec/values/calculation/round/strategy"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("strategy")
}

mod down;

mod nearest;

mod to_zero;

mod up;
