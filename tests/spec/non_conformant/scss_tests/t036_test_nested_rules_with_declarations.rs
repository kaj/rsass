//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/036_test_nested_rules_with_declarations.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar {c: d}\
             \n  a: b}\n"),
        "foo {\
         \n  a: b;\
         \n}\
         \nfoo bar {\
         \n  c: d;\
         \n}\n"
    );
}
