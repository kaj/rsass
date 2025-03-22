//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/012_test_if_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("012_test_if_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if \"foo\" != \"foo\" {foo {a: b}}\
             \n@else if \"foo\" == \"foo\" {bar {a: b}}\
             \n@else if true {baz {a: b}}\n"),
        "bar {\
         \n  a: b;\
         \n}\n"
    );
}
