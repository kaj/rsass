//! Tests auto-converted from "sass-spec/spec/values/maps"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("maps")
}

mod duplicate_keys;

mod errors;

mod invalid_key;

mod key_equality;

mod length;

mod map_values;
