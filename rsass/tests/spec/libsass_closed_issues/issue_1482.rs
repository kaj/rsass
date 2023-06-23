//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1482.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1482")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".mango {\
             \n  color: red;\
             \n}\n\
             \ntype {\
             \n  &__else {\
             \n    @extend .mango;\
             \n  }\
             \n}\n\
             \n.qualified {\
             \n  &__else {\
             \n    @extend .mango;\
             \n  }\
             \n}\n"),
        ".mango, .qualified__else, type__else {\
         \n  color: red;\
         \n}\n"
    );
}
