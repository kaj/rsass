//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1960.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1960")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("foo:not(.missing) {\
             \n  a: b;\n\
             \n  &:hover { c: d; }\
             \n}\n\
             \nbar {\
             \n  @extend .missing;\
             \n}"),
        "foo:not(.missing):not(bar) {\
         \n  a: b;\
         \n}\
         \nfoo:not(.missing):not(bar):hover {\
         \n  c: d;\
         \n}\n"
    );
}
