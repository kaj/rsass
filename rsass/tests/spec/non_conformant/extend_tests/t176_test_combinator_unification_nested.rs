//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/176_test_combinator_unification_nested.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("176_test_combinator_unification_nested")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a > .b + x {a: b}\
             \n.c > .d + y {@extend x}\n"),
        ".a > .b + x, .c.a > .d.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
