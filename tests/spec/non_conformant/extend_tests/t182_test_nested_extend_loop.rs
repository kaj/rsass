//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/182_test_nested_extend_loop.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".bar {\
             \na: b;\
             \n.foo {c: d; @extend .bar}\
             \n}\n"),
        ".bar, .bar .foo {\
         \n  a: b;\
         \n}\
         \n.bar .foo {\
         \n  c: d;\
         \n}\n"
    );
}
