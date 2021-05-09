//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/109_test_selector_interpolation_in_string.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo[val=\"bar #{\"foo\" + \" bar\"} baz\"] {a: b}\n"),
        "foo[val=\"bar foo bar baz\"] {\
         \n  a: b;\
         \n}\n"
    );
}
