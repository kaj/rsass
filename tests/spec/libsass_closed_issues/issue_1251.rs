//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1251.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1251")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  yellow: yellow;\
             \n  red: red;\
             \n  blue: blue;\
             \n  white: white;\
             \n  black: black;\
             \n  eval: if(red, yellow, null);\
             \n}\n"),
        ".foo {\
         \n  yellow: yellow;\
         \n  red: red;\
         \n  blue: blue;\
         \n  white: white;\
         \n  black: black;\
         \n  eval: yellow;\
         \n}\n"
    );
}
