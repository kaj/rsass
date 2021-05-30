//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/002_test_basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".bar {@extend .foo}\
             \n.foo {a: b}\n"),
        ".foo, .bar {\
         \n  a: b;\
         \n}\n"
    );
}
