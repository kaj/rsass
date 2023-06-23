//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2017.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2017")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("foo {\r\
             \n  bar: baz;\r\
             \n}\r\
             \n\r\
             \n@mixin link() {\r\
             \n  a:not(.btn) {\r\
             \n    color: red;\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \nfoo {\r\
             \n  @include link;\r\
             \n  @extend .btn;\r\
             \n  @include link;\r\
             \n}"),
        "foo {\
         \n  bar: baz;\
         \n}\
         \nfoo a:not(.btn):not(foo) {\
         \n  color: red;\
         \n}\
         \nfoo a:not(.btn):not(foo) {\
         \n  color: red;\
         \n}\n"
    );
}
