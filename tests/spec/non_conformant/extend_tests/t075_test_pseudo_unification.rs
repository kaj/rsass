//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/075_test_pseudo_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("075_test_pseudo_unification")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a .baz:foo {a: b}\
             \n:after {@extend .baz} -a {@extend %-a}\n"),
        "-a .baz:foo, -a :foo:after {\
         \n  a: b;\
         \n}\n"
    );
}
