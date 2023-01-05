//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/032_test_nested_rules.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("032_test_nested_rules")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {bar {a: b}}\n"),
        "foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
