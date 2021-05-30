//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/101_test_nested_extender_unifies_common_substring.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            ".baz .bip .bap .bink .foo {a: b}\
             \n.brat .bip .bap bar {@extend .foo}\n"
        ),
        ".baz .bip .bap .bink .foo, .baz .brat .bip .bap .bink bar, .brat .baz .bip .bap .bink bar {\
         \n  a: b;\
         \n}\n"
    );
}
