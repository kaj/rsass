//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/003_test_basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \n.bar {c: d; @extend .foo}\n"),
        ".foo, .bar {\
         \n  a: b;\
         \n}\
         \n.bar {\
         \n  c: d;\
         \n}\n"
    );
}
