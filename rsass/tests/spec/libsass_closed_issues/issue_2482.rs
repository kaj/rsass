//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2482.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2482")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@mixin mixin()\
             \n{\
             \n\t& .class\
             \n\t{\
             \n\t\tcolor: black;\
             \n\t}\
             \n}\n\
             \n@include mixin();"),
        "& .class {\
         \n  color: black;\
         \n}\n"
    );
}
