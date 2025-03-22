//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/122_test_directive_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("122_test_directive_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$baz: 12;\
             \n@foo bar#{$baz} qux {a: b}\n"),
        "@foo bar12 qux {\
         \n  a: b;\
         \n}\n"
    );
}
