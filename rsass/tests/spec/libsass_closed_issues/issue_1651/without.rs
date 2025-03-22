//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1651/without.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("without")
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
