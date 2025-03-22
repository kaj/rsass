//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/013_test_if_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("013_test_if_directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if \"foo\" != \"foo\" {foo {a: b}}\
             \n@else {bar {a: b}}\n"),
        "bar {\
         \n  a: b;\
         \n}\n"
    );
}
