//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/172_test_combinator_unification_plus_space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a + x {a: b}\
             \n.b y {@extend x}\n"),
        ".a + x, .b .a + y {\
         \n  a: b;\
         \n}\n"
    );
}
