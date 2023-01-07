//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2376.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2376")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".test {\r\
             \n\tbackground:url(//img12.360buyimg.com/..);\r\
             \n\t.a{\r\
             \n\t\theight: 100px;\r\
             \n\t}\r\
             \n}"),
        ".test {\
         \n  background: url(//img12.360buyimg.com/..);\
         \n}\
         \n.test .a {\
         \n  height: 100px;\
         \n}\n"
    );
}
