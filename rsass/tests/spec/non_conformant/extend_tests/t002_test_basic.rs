//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/002_test_basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("002_test_basic")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".bar {@extend .foo}\
             \n.foo {a: b}\n"),
        ".foo, .bar {\
         \n  a: b;\
         \n}\n"
    );
}
