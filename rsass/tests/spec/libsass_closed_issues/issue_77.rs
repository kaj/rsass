//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_77.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_77")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin m {\r\
             \n  .m {\r\
             \n    color: red;\r\
             \n    @content;\r\
             \n  }\r\
             \n}\r\
             \ndiv.a {\r\
             \n  @include m;\r\
             \n}"),
        "div.a .m {\
         \n  color: red;\
         \n}\n"
    );
}
