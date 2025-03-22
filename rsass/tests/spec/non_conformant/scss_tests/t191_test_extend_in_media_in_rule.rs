//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/191_test_extend_in_media_in_rule.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("191_test_extend_in_media_in_rule")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @media screen {\
             \n    @extend %bar;\
             \n  }\
             \n}\n\
             \n@media screen {\
             \n  %bar {\
             \n    a: b;\
             \n  }\
             \n}\n"),
        "@media screen {\
         \n  .foo {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
