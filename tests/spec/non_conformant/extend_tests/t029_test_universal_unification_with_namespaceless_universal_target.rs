//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/029_test_universal_unification_with_namespaceless_universal_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a *.foo {a: b}\
             \nns|* {@extend .foo} -a {@extend %-a}\n"),
        "-a *.foo {\
         \n  a: b;\
         \n}\n"
    );
}
