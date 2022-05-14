//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/011_test_if_directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("011_test_if_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if \"foo\" == \"foo\" {foo {a: b}}\
             \n@if \"foo\" != \"foo\" {bar {a: b}}\n"),
        "foo {\
         \n  a: b;\
         \n}\n"
    );
}
