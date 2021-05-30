//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/046_test_element_unification_with_simple_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.bar {a: b}\
             \nns|a {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo.bar, -a ns|a.bar {\
         \n  a: b;\
         \n}\n"
    );
}
