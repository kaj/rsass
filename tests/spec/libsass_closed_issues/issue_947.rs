//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_947.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@keyframes test {\
             \n  $var: 10%;\
             \n  #{$var} {\
             \n    color: red;\
             \n  }\
             \n}\n"),
        "@keyframes test {\
         \n  10% {\
         \n    color: red;\
         \n  }\
         \n}\n"
    );
}
