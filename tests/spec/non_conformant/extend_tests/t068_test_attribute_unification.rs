//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/068_test_attribute_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a [foo=bar].baz {a: b}\
             \n[ns|foo=bar] {@extend .baz} -a {@extend %-a}\n"),
        "-a [foo=bar].baz, -a [foo=bar][ns|foo=bar] {\
         \n  a: b;\
         \n}\n"
    );
}
