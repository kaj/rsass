//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_941.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_941")
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
         \n.two { /* 3 */\
         \n  color: #F00; /* 4 */\
         \n}\n"
    );
}
