//! Tests auto-converted from "sass-spec/spec/directives/use/with/used_in_input.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("used_in_input")
        .mock_file("_other.scss", "$a: original !default;\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured);\
             \nb {c: other.$a}\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
