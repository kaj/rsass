//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/009_test_multiple_extends_with_multiple_extenders_and_single_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo .bar {a: b}\
             \n.baz {@extend .foo}\
             \n.bang {@extend .bar}\n"),
        ".foo .bar, .foo .bang, .baz .bar, .baz .bang {\
         \n  a: b;\
         \n}\n"
    );
}
