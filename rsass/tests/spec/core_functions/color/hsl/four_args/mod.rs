//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("four_args")
}

mod alpha;

mod in_gamut;

mod out_of_gamut;

mod special_functions;
