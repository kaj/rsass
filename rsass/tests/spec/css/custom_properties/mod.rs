//! Tests auto-converted from "sass-spec/spec/css/custom_properties"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("custom_properties")
}

mod empty;

mod error;

mod exclamation;

mod indentation;

mod name_interpolation;

mod nesting_characters;

mod script;

mod simple;

mod strings;

mod syntax;

mod trailing_comment;

mod trailing_whitespace;

mod value_interpolation;

mod without_semicolon;
