//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/11_escaped_literal"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("11_escaped_literal")
}

mod t01_inline;

mod t02_variable;

mod t03_inline_double;

mod t04_variable_double;

mod t05_variable_quoted_double;

mod t06_escape_interpolation;
