//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/065_test_attribute_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("065_test_attribute_unification")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a [foo=bar].baz {a: b}\
             \n[foo=baz] {@extend .baz} -a {@extend %-a}\n"),
        "-a [foo=bar].baz, -a [foo=bar][foo=baz] {\
         \n  a: b;\
         \n}\n"
    );
}
