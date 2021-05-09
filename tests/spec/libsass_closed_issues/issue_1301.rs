//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1301.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$name: \"my-class\";\n\
             \n.-#{$name} {\
             \n  content: \"test\";\
             \n}\n"),
        ".-my-class {\
         \n  content: \"test\";\
         \n}\n"
    );
}
