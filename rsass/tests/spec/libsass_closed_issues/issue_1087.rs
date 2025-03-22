//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1087.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1087")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: bar;\
             \na {\
             \n  foo: url($foo);\
             \n  foo: url(#{$foo});\
             \n}\n"),
        "a {\
         \n  foo: url(bar);\
         \n  foo: url(bar);\
         \n}\n"
    );
}
