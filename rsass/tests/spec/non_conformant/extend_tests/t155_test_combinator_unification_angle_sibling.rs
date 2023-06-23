//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/155_test_combinator_unification_angle_sibling.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("155_test_combinator_unification_angle_sibling")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a + x {a: b}\
             \n.b > y {@extend x}\n"),
        ".a + x, .b > .a + y {\
         \n  a: b;\
         \n}\n"
    );
}
