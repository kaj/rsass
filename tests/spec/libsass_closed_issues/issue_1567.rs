//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1567.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1567")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("/* any */@media/* first */\
             \n/* screen */screen /*something */ , /* else */\
             \n/* not */not/* print */print /* final */ {  /* whatever */\
             \n    body { line-height: 1.2 }\
             \n}\n"),
        "/* any */\
         \n@media screen, not print { /* whatever */\
         \n  body {\
         \n    line-height: 1.2;\
         \n  }\
         \n}\n"
    );
}
