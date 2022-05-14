//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/076_test_pseudo_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("076_test_pseudo_unification")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a .baz:after {a: b}\
             \n:foo {@extend .baz} -a {@extend %-a}\n"),
        "-a .baz:after, -a :foo:after {\
         \n  a: b;\
         \n}\n"
    );
}
