//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/066_test_attribute_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a [foo=bar].baz {a: b}\
             \n[foo^=bar] {@extend .baz} -a {@extend %-a}\n"),
        "-a [foo=bar].baz, -a [foo=bar][foo^=bar] {\
         \n  a: b;\
         \n}\n"
    );
}
