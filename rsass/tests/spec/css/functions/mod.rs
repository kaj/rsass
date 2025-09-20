//! Tests auto-converted from "sass-spec/spec/css/functions"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("functions")
}

mod error;

mod not_special;

mod special;

mod url;

mod var;
