//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/133_test_combinator_unification_for_hacky_combinators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("133_test_combinator_unification_for_hacky_combinators")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a > + x {a: b}\
             \n.b y {@extend x}\n"),
        ".a > + x, .a .b > + y, .b .a > + y {\
         \n  a: b;\
         \n}\n"
    );
}
