//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/179_test_extend_self_loop.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b; @extend .foo}\n"),
        ".foo {\
         \n  a: b;\
         \n}\n"
    );
}
