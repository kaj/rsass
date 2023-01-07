//! Tests auto-converted from "sass-spec/spec/non_conformant/sass/import/unquoted.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("unquoted")
        .mock_file("sub/unquoted.scss", "d {e: f}\n")
        .mock_file("unquoted.scss", "a {b: c}\n")
}
