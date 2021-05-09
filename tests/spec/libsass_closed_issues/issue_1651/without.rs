//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1651/without.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a {\
             \n  display: block;\
             \n}\n\
             \n.b {\
             \n  @at-root (without: media) {\
             \n    @extend .a;\
             \n  }\
             \n} \n"),
        ".a, .b {\
         \n  display: block;\
         \n}\n"
    );
}
