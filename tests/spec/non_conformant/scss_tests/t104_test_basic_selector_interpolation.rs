//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/104_test_basic_selector_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo#{1 + 2} baz {a: b}\n"),
        "foo3 baz {\
         \n  a: b;\
         \n}\n"
    );
}
