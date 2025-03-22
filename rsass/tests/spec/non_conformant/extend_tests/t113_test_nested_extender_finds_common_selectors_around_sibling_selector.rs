//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/113_test_nested_extender_finds_common_selectors_around_sibling_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("113_test_nested_extender_finds_common_selectors_around_sibling_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a ~ b c .c1 {a: b}\
             \na b .c2 {@extend .c1}\n"),
        "a ~ b c .c1, a a ~ b c .c2 {\
         \n  a: b;\
         \n}\n"
    );
}
