//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/101_test_nested_extender_unifies_common_substring.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("101_test_nested_extender_unifies_common_substring")
}

#[test]
#[ignore] // unexepected error
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
