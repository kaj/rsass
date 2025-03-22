//! Tests auto-converted from "sass-spec/spec/non_conformant"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("non_conformant")
}

mod basic;

mod colors;

mod errors;

mod extend_tests;

mod media_import;

mod misc;

mod mixin;

mod nesting;

mod operations;

mod parser;

// Ignoring "sass", not expected to work yet.

mod sass_4_0;

mod scope;

mod scss;

mod scss_tests;

mod variables;
