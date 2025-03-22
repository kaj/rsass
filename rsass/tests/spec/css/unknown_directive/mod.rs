//! Tests auto-converted from "sass-spec/spec/css/unknown_directive"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unknown_directive")
}

mod comment;

mod error;

mod name_interpolation;

mod plain;

mod value_interpolation;

mod whitespace;
