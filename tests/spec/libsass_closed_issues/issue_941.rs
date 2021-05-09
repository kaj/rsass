//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_941.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".one, /* 1 */\
             \n.two /* 2 */ { /* 3 */\
             \n\tcolor: #F00; /* 4 */\
             \n}\n"),
        ".one,\
         \n.two {\
         \n  /* 3 */\
         \n  color: #F00;\
         \n  /* 4 */\
         \n}\n"
    );
}
