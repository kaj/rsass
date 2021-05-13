//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/035_test_nested_rules_with_declarations.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: b;\
             \n  bar {c: d}}\n"),
        "foo {\
         \n  a: b;\
         \n}\
         \nfoo bar {\
         \n  c: d;\
         \n}\n"
    );
}
