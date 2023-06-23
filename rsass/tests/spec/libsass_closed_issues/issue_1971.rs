//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1971.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1971")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%foo1 {\
             \n  @supports (flex-wrap: wrap) {\
             \n    flex: auto;\
             \n  }\
             \n}\n\
             \n@supports (flex-wrap: wrap) {\
             \n  %foo2 {\
             \n    flex: auto;\
             \n  }\
             \n}\n\
             \n.bar {\
             \n  @extend %foo1;\
             \n  @extend %foo2;\
             \n}\n"),
        "@supports (flex-wrap: wrap) {\
         \n  .bar {\
         \n    flex: auto;\
         \n  }\
         \n}\
         \n@supports (flex-wrap: wrap) {\
         \n  .bar {\
         \n    flex: auto;\
         \n  }\
         \n}\n"
    );
}
