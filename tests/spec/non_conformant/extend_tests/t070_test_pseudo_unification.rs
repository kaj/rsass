//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/070_test_pseudo_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("070_test_pseudo_unification")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a :foo.baz {a: b}\
             \n:foo(2n+1) {@extend .baz} -a {@extend %-a}\n"),
        "-a :foo.baz, -a :foo:foo(2n+1) {\
         \n  a: b;\
         \n}\n"
    );
}
