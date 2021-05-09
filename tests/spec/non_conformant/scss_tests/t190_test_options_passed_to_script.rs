//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/190_test_options_passed_to_script.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {color: darken(black, 10%)}\n"),
        "foo {\
         \n  color: black;\
         \n}\n"
    );
}
