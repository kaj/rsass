//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/082_test_not_remains_at_end_of_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("082_test_not_remains_at_end_of_selector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo:not(.bar) {a: b}\
             \n.baz {@extend .foo}\n"),
        ".foo:not(.bar), .baz:not(.bar) {\
         \n  a: b;\
         \n}\n"
    );
}
