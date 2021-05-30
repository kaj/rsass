//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/158_test_combinator_unification_double_angle.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a > x {a: b}\
             \n.b > y {@extend x}\n"),
        ".a > x, .b.a > y {\
         \n  a: b;\
         \n}\n"
    );
}
