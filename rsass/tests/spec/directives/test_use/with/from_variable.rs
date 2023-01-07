//! Tests auto-converted from "sass-spec/spec/directives/use/with/from_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("from_variable")
        .mock_file("_other.scss", "$a: original a !default;\nb {c: $a}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$a: configured;\
             \n@use \"other\" with ($a: $a);\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
