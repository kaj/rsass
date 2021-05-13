//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/162_test_combinator_unification_double_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a + x {a: b}\
             \n.b + y {@extend x}\n"),
        ".a + x, .b.a + y {\
         \n  a: b;\
         \n}\n"
    );
}
