//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/149_test_combinator_unification_tilde_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a ~ x {a: b}\
             \n.a.b + y {@extend x}\n"),
        ".a ~ x, .a.b + y {\
         \n  a: b;\
         \n}\n"
    );
}
