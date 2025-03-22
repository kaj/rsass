//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_552.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_552")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a,\
             \ndiv {\
             \n    top: 0;\
             \n}\n\
             \n.a,\
             \n.b {\
             \n    &.c {\
             \n        color: red;\
             \n    }\
             \n}\n"),
        "a,\
         \ndiv {\
         \n  top: 0;\
         \n}\
         \n.a.c,\
         \n.b.c {\
         \n  color: red;\
         \n}\n"
    );
}
