//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/175_test_combinator_unification_plus_space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("175_test_combinator_unification_plus_space")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a x {a: b}\
             \n.b + y {@extend x}\n"),
        ".a x, .a .b + y {\
         \n  a: b;\
         \n}\n"
    );
}
