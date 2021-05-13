//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/025_test_universal_unification_with_namespaceless_universal_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a *.foo {a: b}\
             \n* {@extend .foo} -a {@extend %-a}\n"),
        "-a * {\
         \n  a: b;\
         \n}\n"
    );
}
