//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/015_test_target_with_child.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo .bar {a: b}\
             \n.baz {@extend .foo}\n"),
        ".foo .bar, .baz .bar {\
         \n  a: b;\
         \n}\n"
    );
}
