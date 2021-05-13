//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/144_test_combinator_unification_tilde_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a.b + x {a: b}\
             \n.a ~ y {@extend x}\n"),
        ".a.b + x, .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
