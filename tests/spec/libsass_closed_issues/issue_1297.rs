//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1297.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".test .testa {\
             \n  @at-root #{\"%foo\"} {\
             \n    color: red;\
             \n  }\
             \n  @extend %foo;\
             \n}\n"),
        ".test .testa {\
         \n  color: red;\
         \n}\n"
    );
}
