//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/099_test_nested_extender_alternates_parents.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            ".baz .bip .foo {a: b}\
             \nfoo .grank bar {@extend .foo}\n"
        ),
        ".baz .bip .foo, .baz .bip foo .grank bar, foo .grank .baz .bip bar {\
         \n  a: b;\
         \n}\n"
    );
}
