//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1214.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin keyframes($animation-name) {\
             \n  @keyframes $animation-name {\
             \n    @content;\
             \n  }\
             \n}\n\
             \n@include keyframes(bounce) {\
             \n  0%, 20%, 50%, 80%, 100% {transform: translateY(0);}\
             \n  40% {transform: translateY(-30px);}\
             \n  60% {transform: translateY(-15px);}\
             \n}"),
        "@keyframes $animation-name {\
         \n  0%, 20%, 50%, 80%, 100% {\
         \n    transform: translateY(0);\
         \n  }\
         \n  40% {\
         \n    transform: translateY(-30px);\
         \n  }\
         \n  60% {\
         \n    transform: translateY(-15px);\
         \n  }\
         \n}\n"
    );
}
