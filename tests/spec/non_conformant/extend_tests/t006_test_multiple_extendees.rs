//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/006_test_multiple_extendees.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("006_test_multiple_extendees")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \n.bar {c: d}\
             \n.baz {@extend .foo; @extend .bar}\n"),
        ".foo, .baz {\
         \n  a: b;\
         \n}\
         \n.bar, .baz {\
         \n  c: d;\
         \n}\n"
    );
}
