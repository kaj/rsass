//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/if_directive.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("if_directive")
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
