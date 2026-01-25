//! Tests auto-converted from "sass-spec/spec/expressions/if/error"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod and;

mod empty;

mod invalid_function_name;

mod missing;

mod missing_whitepsace;

mod not;

mod or;

mod paren;

mod raw;

mod semicolon;
