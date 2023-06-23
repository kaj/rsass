//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/004_test_basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("004_test_basic")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \n.bar {@extend .foo; c: d}\n"),
        ".foo, .bar {\
         \n  a: b;\
         \n}\
         \n.bar {\
         \n  c: d;\
         \n}\n"
    );
}
