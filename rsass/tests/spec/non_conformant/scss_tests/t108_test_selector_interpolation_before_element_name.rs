//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/108_test_selector_interpolation_before_element_name.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("108_test_selector_interpolation_before_element_name")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#{\"foo\" + \" bar\"}baz {a: b}\n"),
        "foo barbaz {\
         \n  a: b;\
         \n}\n"
    );
}
