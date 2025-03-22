//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/047_test_unknown_directive_bubbling.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("047_test_unknown_directive_bubbling")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @fblthp {\
             \n    .bar {a: b}\
             \n  }\
             \n}\n"),
        "@fblthp {\
         \n  .foo .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
