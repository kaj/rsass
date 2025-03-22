//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/110_test_selector_interpolation_in_pseudoclass.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("110_test_selector_interpolation_in_pseudoclass")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo:nth-child(#{5 + \"n\"}) {a: b}\n"),
        "foo:nth-child(5n) {\
         \n  a: b;\
         \n}\n"
    );
}
