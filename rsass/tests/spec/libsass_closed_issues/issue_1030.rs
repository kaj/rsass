//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1030.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1030")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin will-change() {\
             \n  @supports (will-change: transform) {\
             \n    will-change: transform;\
             \n  }\
             \n}\
             \ndiv {\
             \n  a {\
             \n    top: 10px;\
             \n    @include will-change();\
             \n  }\
             \n}\n"),
        "div a {\
         \n  top: 10px;\
         \n}\
         \n@supports (will-change: transform) {\
         \n  div a {\
         \n    will-change: transform;\
         \n  }\
         \n}\n"
    );
}
