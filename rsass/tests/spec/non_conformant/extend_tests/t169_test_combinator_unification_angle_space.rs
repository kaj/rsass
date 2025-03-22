//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/169_test_combinator_unification_angle_space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("169_test_combinator_unification_angle_space")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a x {a: b}\
             \n.b > y {@extend x}\n"),
        ".a x, .a .b > y {\
         \n  a: b;\
         \n}\n"
    );
}
