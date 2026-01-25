//! Tests auto-converted from "sass-spec/spec/directives/import/configuration"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("configuration")
}

mod import_twice;

mod indirect;

mod midstream_definition;

mod nested;

mod prefixed_as;

mod same_file;

mod separate_file;

mod unrelated_variable;
