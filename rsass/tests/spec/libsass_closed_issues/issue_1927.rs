//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1927.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1927")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@media screen {\
             \n  $variable: dynamic;\
             \n  .foo-#{$variable} {a: b}\
             \n  .bar {\
             \n    @extend .foo-dynamic\
             \n  }\
             \n}"),
        "@media screen {\
         \n  .foo-dynamic, .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
