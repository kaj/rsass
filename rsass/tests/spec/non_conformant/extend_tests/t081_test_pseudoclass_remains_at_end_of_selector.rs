//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/081_test_pseudoclass_remains_at_end_of_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("081_test_pseudoclass_remains_at_end_of_selector")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a.foo:bar {a: b}\
             \n.baz {@extend .foo}\n"),
        "a.foo:bar, a.baz:bar {\
         \n  a: b;\
         \n}\n"
    );
}
