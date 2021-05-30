//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/177_test_combinator_unification_nested.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a > .b + x {a: b}\
             \n.c > y {@extend x}\n"),
        ".a > .b + x, .c.a > .b + y {\
         \n  a: b;\
         \n}\n"
    );
}
