//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("directive")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@media all {\
             \n  .foo {\
             \n\tcolor: red;\
             \n  }\
             \n}"),
        "@media all {\
         \n  .foo {\
         \n    color: red;\
         \n  }\
         \n}\n"
    );
}
