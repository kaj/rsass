//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("expression")
}

mod calculation;

mod function;

mod interpolation;

mod list;

mod map;

mod operation;

mod parent_selector;

mod parentheses;

mod variable;
