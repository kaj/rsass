//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/106_test_basic_selector_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("106_test_basic_selector_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#{\"foo\"}.bar baz {a: b}\n"),
        "foo.bar baz {\
         \n  a: b;\
         \n}\n"
    );
}
