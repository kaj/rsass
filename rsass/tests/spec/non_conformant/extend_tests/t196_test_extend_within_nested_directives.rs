//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/196_test_extend_within_nested_directives.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("196_test_extend_within_nested_directives")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@media screen {\
             \n@flooblehoof {\
             \n  .foo {a: b}\
             \n  .bar {@extend .foo}\
             \n}\
             \n}\n"),
        "@media screen {\
         \n  @flooblehoof {\
         \n    .foo, .bar {\
         \n      a: b;\
         \n    }\
         \n  }\
         \n}\n"
    );
}
