//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/036_test_nested_rules_with_declarations.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("036_test_nested_rules_with_declarations")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar {c: d}\
             \n  a: b}\n"),
        "foo bar {\
         \n  c: d;\
         \n}\
         \nfoo {\
         \n  a: b;\
         \n}\n"
    );
}
