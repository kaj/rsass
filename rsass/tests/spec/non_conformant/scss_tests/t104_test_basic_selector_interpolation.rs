//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/104_test_basic_selector_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("104_test_basic_selector_interpolation")
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
