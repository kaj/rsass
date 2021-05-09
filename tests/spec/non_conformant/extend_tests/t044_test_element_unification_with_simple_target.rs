//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/044_test_element_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.bar {a: b}\
             \na {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo.bar, -a a.bar {\
         \n  a: b;\
         \n}\n"
    );
}
