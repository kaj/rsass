//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/077_test_pseudo_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a :foo.baz {a: b}\
             \n:foo {@extend .baz} -a {@extend %-a}\n"),
        "-a :foo {\
         \n  a: b;\
         \n}\n"
    );
}
