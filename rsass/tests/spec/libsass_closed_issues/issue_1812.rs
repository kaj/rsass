//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1812.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1812")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@svg-load test url(foo.svg) {\r\
             \n  fill: red;\r\
             \n}\r\
             \n\r\
             \n.foo {\r\
             \n  background: svg-inline(test);\r\
             \n}"),
        "@svg-load test url(foo.svg) {\
         \n  fill: red;\
         \n}\
         \n.foo {\
         \n  background: svg-inline(test);\
         \n}\n"
    );
}
