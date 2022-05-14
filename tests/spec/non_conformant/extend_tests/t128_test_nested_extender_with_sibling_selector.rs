//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/128_test_nested_extender_with_sibling_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("128_test_nested_extender_with_sibling_selector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".baz .foo {a: b}\
             \nfoo + bar {@extend .foo}\n"),
        ".baz .foo, .baz foo + bar {\
         \n  a: b;\
         \n}\n"
    );
}
