//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/007_test_multiple_extends_with_single_extender_and_single_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo .bar {a: b}\
             \n.baz {@extend .foo; @extend .bar}\n"),
        ".foo .bar, .foo .baz, .baz .bar, .baz .baz {\
         \n  a: b;\
         \n}\n"
    );
}
