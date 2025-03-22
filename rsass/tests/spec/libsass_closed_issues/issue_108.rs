//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_108.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_108")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$a: red;\r\
             \n\r\
             \n@mixin f($a: $a) {\r\
             \n  color: $a;\r\
             \n}\r\
             \n\r\
             \nh1 {\r\
             \n  @include f;\r\
             \n}\r\
             \n\r\
             \nh2 {\r\
             \n  @include f(blue);\r\
             \n}"),
        "h1 {\
         \n  color: red;\
         \n}\
         \nh2 {\
         \n  color: blue;\
         \n}\n"
    );
}
