//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/043_test_element_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("043_test_element_unification_with_simple_target")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo {a: b}\
             \na {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo, -a a {\
         \n  a: b;\
         \n}\n"
    );
}
