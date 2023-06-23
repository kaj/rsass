//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/071_test_pseudo_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("071_test_pseudo_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a :foo.baz {a: b}\
             \n::foo {@extend .baz} -a {@extend %-a}\n"),
        "-a :foo.baz, -a :foo::foo {\
         \n  a: b;\
         \n}\n"
    );
}
