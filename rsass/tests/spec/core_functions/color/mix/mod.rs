//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mix")
}

mod alpha;

mod both_weights;

mod error;

mod explicit_method;

mod explicit_weight;

mod hue_interpolation;

mod missing;

mod mixed_spaces;

mod named;

mod predefined;

mod units;

mod unweighted;
