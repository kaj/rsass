//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/130_test_nested_extender_with_hacky_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".baz .foo {a: b}\
             \n> > bar {@extend .foo}\n"),
        ".baz .foo, > > .baz bar {\
         \n  a: b;\
         \n}\n"
    );
}
