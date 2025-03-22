//! Tests auto-converted from "sass-spec/spec/non_conformant/errors"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("errors")
}

mod extend;

mod fn_change_color_1;

mod fn_debug;

mod fn_error;

mod fn_varargs;

mod import;

mod interpolation;

mod invalid_operation;

mod invalid_parent;

mod unicode;
