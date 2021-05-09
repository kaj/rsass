//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/033_test_nested_rules.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar {a: b}\
             \n  baz {b: c}}\n"),
        "foo bar {\
         \n  a: b;\
         \n}\
         \nfoo baz {\
         \n  b: c;\
         \n}\n"
    );
}
