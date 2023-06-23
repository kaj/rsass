//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_950.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_950")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".selector1{ foo: bar; }\
             \n.selector2{ zapf: dings; }\n\
             \n.selector3{ @extend .selector1, .selector2; }"),
        ".selector1, .selector3 {\
         \n  foo: bar;\
         \n}\
         \n.selector2, .selector3 {\
         \n  zapf: dings;\
         \n}\n"
    );
}
