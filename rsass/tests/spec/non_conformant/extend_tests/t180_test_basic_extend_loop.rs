//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/180_test_basic_extend_loop.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("180_test_basic_extend_loop")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b; @extend .bar}\
             \n.bar {c: d; @extend .foo}\n"),
        ".foo, .bar {\
         \n  a: b;\
         \n}\
         \n.bar, .foo {\
         \n  c: d;\
         \n}\n"
    );
}
