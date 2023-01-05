//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/074_test_pseudo_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("074_test_pseudo_unification")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a :foo.baz {a: b}\
             \n:bar {@extend .baz} -a {@extend %-a}\n"),
        "-a :foo.baz, -a :foo:bar {\
         \n  a: b;\
         \n}\n"
    );
}
