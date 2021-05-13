//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/194_test_extend_within_media.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@media screen {\
             \n.foo {a: b}\
             \n.bar {@extend .foo}\
             \n}\n"),
        "@media screen {\
         \n  .foo, .bar {\
         \n    a: b;\
         \n  }\
         \n}\n"
    );
}
