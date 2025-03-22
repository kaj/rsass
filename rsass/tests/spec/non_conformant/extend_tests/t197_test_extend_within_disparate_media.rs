//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/197_test_extend_within_disparate_media.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("197_test_extend_within_disparate_media")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@media screen {.foo {a: b}}\
             \n@media screen {.bar {@extend .foo}}\n"),
        "@media screen {\
         \n  .foo, .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
