//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/090_test_comma_extendee.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {a: b}\
             \n.bar {c: d}\
             \n.baz {@extend .foo, .bar}\n"),
        ".foo, .baz {\
         \n  a: b;\
         \n}\
         \n.bar, .baz {\
         \n  c: d;\
         \n}\n"
    );
}
