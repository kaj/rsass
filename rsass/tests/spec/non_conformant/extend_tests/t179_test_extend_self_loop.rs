//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/179_test_extend_self_loop.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("179_test_extend_self_loop")
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
