//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/046_test_element_unification_with_simple_target.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("046_test_element_unification_with_simple_target")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a .foo.bar {a: b}\
             \nns|a {@extend .foo} -a {@extend %-a}\n"),
        "-a .foo.bar, -a ns|a.bar {\
         \n  a: b;\
         \n}\n"
    );
}
