//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1025.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1025")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin m() {\
             \n  .a & {\
             \n    @content;\
             \n  }\
             \n}\n\
             \n:not(:last-of-type) {\
             \n  top: 10px;\
             \n  @include m {\
             \n    top: 10px;\
             \n  }\
             \n}\n"),
        ":not(:last-of-type) {\
         \n  top: 10px;\
         \n}\
         \n.a :not(:last-of-type) {\
         \n  top: 10px;\
         \n}\n"
    );
}
