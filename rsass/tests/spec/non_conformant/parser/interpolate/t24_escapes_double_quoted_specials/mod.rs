//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/24_escapes_double_quoted_specials"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("24_escapes_double_quoted_specials")
}

mod t01_inline;

mod t02_variable;

mod t03_inline_double;

mod t04_variable_double;

mod t06_escape_interpolation;

mod todo_05_variable_quoted_double;
