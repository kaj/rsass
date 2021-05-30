//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/163_test_combinator_unification_double_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a.a + x {a: b}\
             \nb.b + y {@extend x}\n"),
        "a.a + x {\
         \n  a: b;\
         \n}\n"
    );
}
