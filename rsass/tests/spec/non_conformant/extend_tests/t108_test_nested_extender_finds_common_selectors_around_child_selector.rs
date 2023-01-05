//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/108_test_nested_extender_finds_common_selectors_around_child_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("108_test_nested_extender_finds_common_selectors_around_child_selector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a > b c .c1 {a: b}\
             \nb c .c2 {@extend .c1}\n"),
        "a > b c .c1, a > b c .c2 {\
         \n  a: b;\
         \n}\n"
    );
}
