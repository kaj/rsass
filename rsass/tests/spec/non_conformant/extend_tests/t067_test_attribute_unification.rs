//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/067_test_attribute_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("067_test_attribute_unification")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a [foo=bar].baz {a: b}\
             \n[foot=bar] {@extend .baz} -a {@extend %-a}\n"),
        "-a [foo=bar].baz, -a [foo=bar][foot=bar] {\
         \n  a: b;\
         \n}\n"
    );
}
