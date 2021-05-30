//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/043_test_element_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a .foo {a: b}\
             \na {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo, -a a {\
         \n  a: b;\
         \n}\n"
    );
}
