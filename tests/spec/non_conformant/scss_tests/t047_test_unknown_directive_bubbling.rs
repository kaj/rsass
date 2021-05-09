//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/047_test_unknown_directive_bubbling.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
