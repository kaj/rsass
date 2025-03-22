//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/error"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod args;

mod incompatible_channel;

mod missing;

mod mixed_formats;

mod space;

mod test_type;

mod units;
