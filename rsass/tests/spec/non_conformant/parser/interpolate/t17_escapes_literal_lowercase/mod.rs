//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("17_escapes_literal_lowercase")
}

mod t01_inline;

mod t02_variable;

mod t03_inline_double;

mod t04_variable_double;

mod t05_variable_quoted_double;

mod t06_escape_interpolation;
