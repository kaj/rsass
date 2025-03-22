//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/107_test_nested_extender_finds_common_selectors_around_child_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("107_test_nested_extender_finds_common_selectors_around_child_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a > b c .c1 {a: b}\
             \na c .c2 {@extend .c1}\n"),
        "a > b c .c1, a > b c .c2 {\
         \n  a: b;\
         \n}\n"
    );
}
