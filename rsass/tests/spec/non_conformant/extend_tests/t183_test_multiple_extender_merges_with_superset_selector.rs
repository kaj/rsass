//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/183_test_multiple_extender_merges_with_superset_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("183_test_multiple_extender_merges_with_superset_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo {@extend .bar; @extend .baz}\
             \na.bar.baz {a: b}\n"),
        "a.bar.baz, a.foo {\
         \n  a: b;\
         \n}\n"
    );
}
