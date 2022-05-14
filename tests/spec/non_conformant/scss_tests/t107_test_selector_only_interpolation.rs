//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/107_test_selector_only_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("107_test_selector_only_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#{\"foo\" + \" bar\"} {a: b}\n"),
        "foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
