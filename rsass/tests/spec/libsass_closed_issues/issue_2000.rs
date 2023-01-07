//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2000.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2000")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".m__exhibit-header--medium {\
             \n    @extend #{&}--plain;\
             \n    &--plain {\
             \n        font-size: --&;\
             \n    }\
             \n}"),
        ".m__exhibit-header--medium--plain, .m__exhibit-header--medium {\
         \n  font-size: -- .m__exhibit-header--medium--plain;\
         \n}\n"
    );
}
