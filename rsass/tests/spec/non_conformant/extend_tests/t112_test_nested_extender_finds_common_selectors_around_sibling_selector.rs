//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/112_test_nested_extender_finds_common_selectors_around_sibling_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("112_test_nested_extender_finds_common_selectors_around_sibling_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a ~ b c .c1 {a: b}\
             \na c .c2 {@extend .c1}\n"),
        "a ~ b c .c1, a ~ b a c .c2, a a ~ b c .c2 {\
         \n  a: b;\
         \n}\n"
    );
}
