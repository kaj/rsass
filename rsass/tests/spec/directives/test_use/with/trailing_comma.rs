//! Tests auto-converted from "sass-spec/spec/directives/use/with/trailing_comma.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("trailing_comma")
        .mock_file("_other.scss", "$a: original !default;\nb {c: $a}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"other\" with ($a: configured,);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
